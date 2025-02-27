use crate::aggregator::one_inch::OneInchClient;
use crate::common::constants_lib::*;
use crate::common::math_lib::{MathLib, WAD};
use crate::common::shares_math_lib::SharesMathLib;
use crate::oval::oval_client::OvalClient;
use bindings::{
    i_morpho::{Market, MarketParams, Position},
    liquidator::{LiquidationParams, Liquidator},
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, PendingTransaction, Provider},
    signers::{Signer, Wallet},
    types::{Address, Bytes, TxHash, U256, U64},
    utils::{hex::ToHexExt, keccak256},
};
use eyre::{eyre, Result};
use log::{debug, error, info, warn};
use std::sync::Arc;

use super::swapper::{find_swap_params, SwapParams};

const ETH_ADDRESS: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";

pub async fn trigger_liquidation(
    client: Arc<Provider<Http>>,
    liquidator: &Liquidator<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    user: &Address,
    position: &Position,
    market_params: &MarketParams,
    market: &Market,
    collateral_price: &U256,
    one_inch_client: &OneInchClient,
    builder_payment_percent: u8,
    oval_client: &OvalClient,
    block_number: &U64,
    chain_id: u64,
) -> Result<TxHash> {
    let (seized_assets, repaid_debt) =
        calculate_liquidated_amounts(market_params, position, market, collateral_price);

    let swap_params =
        find_swap_params(market_params, seized_assets, one_inch_client, &liquidator.address())
            .await?;

    if repaid_debt > swap_params.swapped_debt {
        return Err(eyre!("Repaid debt is greater than swap result from the collateral"));
    }

    let expected_debt_profit = swap_params.swapped_debt - repaid_debt;

    let debt_quote =
        get_price_in_eth(&market_params.loan_token, &expected_debt_profit, one_inch_client).await?;

    let raw_tx = create_liquidation_tx(
        liquidator,
        market_params,
        user,
        &swap_params,
        &debt_quote,
        builder_payment_percent,
        chain_id,
    )
    .await?;

    // Target the next block when sending the bundled raw transaction over the Oval node.
    oval_client.send_raw_txs_bundle(&vec![raw_tx.clone()], block_number + 1).await?;

    let tx_hash = TxHash(keccak256(raw_tx));

    // Spawn a task to log the result of the liquidation transaction without blocking the main thread.
    tokio::spawn(async move {
        let pending_tx = {
            let provider_ref = Arc::as_ref(&client);
            PendingTransaction::new(tx_hash, provider_ref)
        };
        match pending_tx.await {
            Ok(result) => match result {
                Some(receipt) => {
                    info!("Successful liquidation tx: {:?}", receipt.transaction_hash);
                    debug!("Liquidation tx receipt: {:#?}", receipt);
                }
                None => warn!("Liquidation transaction {:?} was not mined", tx_hash),
            },
            Err(e) => error!("Error sending liquidation tx {:?}: {}", tx_hash, e),
        }
    });

    Ok(tx_hash)
}

// This does not handle errors as it is only used in the context of the liquidation and all the shares math replicates
// the logic from Morpho Blue contract.
fn calculate_liquidated_amounts(
    market_params: &MarketParams,
    position: &Position,
    market: &Market,
    collateral_price: &U256,
) -> (U256, U256) {
    let borrow_shares = U256::from(position.borrow_shares);
    let total_borrow_assets = U256::from(market.total_borrow_assets);
    let total_borrow_shares = U256::from(market.total_borrow_shares);

    // The liquidation incentive factor is min(maxLiquidationIncentiveFactor, 1/(1 - cursor*(1 - lltv))).
    let liquidation_incentive_factor = MAX_LIQUIDATION_INCENTIVE_FACTOR
        .min(WAD.w_div_down(&(WAD - LIQUIDATION_CURSOR.w_mul_down(&(WAD - market_params.lltv)))));

    // Get seized assets as if all the debt was repaid, but limited to the user collateral.
    let seized_assets = borrow_shares
        .to_assets_down(&total_borrow_assets, &total_borrow_shares)
        .w_mul_down(&liquidation_incentive_factor)
        .mul_div_down(&ORACLE_PRICE_SCALE, collateral_price)
        .min(U256::from(position.collateral));

    // Get repaid debt based on actual seized assets. This can be less that user debt if bad debt is incurred.
    let repaid_debt = seized_assets
        .mul_div_up(&collateral_price, &ORACLE_PRICE_SCALE)
        .w_div_up(&liquidation_incentive_factor);

    (seized_assets, repaid_debt)
}

async fn get_price_in_eth(
    token: &Address,
    amount: &U256,
    one_inch_client: &OneInchClient,
) -> Result<U256> {
    if U256::is_zero(amount) {
        return Err(eyre!("Quote input amount is zero"));
    }

    let quote = one_inch_client
        .quote_token(&token.encode_hex_with_prefix(), ETH_ADDRESS, &amount.to_string())
        .await?;

    let quote_amount = U256::from(quote.dst_amount.parse::<u128>()?);

    Ok(quote_amount.w_div_down(amount))
}

async fn create_liquidation_tx(
    liquidator: &Liquidator<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    market_params: &MarketParams,
    user: &Address,
    swap_params: &SwapParams,
    debt_quote: &U256,
    builder_payment_percent: u8,
    chain_id: u64,
) -> Result<Bytes> {
    let liquidation_params = LiquidationParams {
        debt_quote: debt_quote.to_owned(),
        builder_payment_percent: U256::from(builder_payment_percent).w_div_down(&U256::from(100)),
        swapper: swap_params.target,
        swap_data: swap_params.swap_data.clone(),
    };

    let mut tx_request = liquidator
        .liquidate_user(
            market_params.to_owned(),
            user.to_owned(),
            swap_params.seized_assets,
            liquidation_params,
        )
        .tx;

    // Set arbitrary gas limit as estimation could fail due to locked Oval price. Even if some providers support passing
    // state overrides, it is not supported by the ethers library.
    tx_request.set_gas(U256::from(1_000_000));

    tx_request.set_chain_id(chain_id);

    liquidator.client().fill_transaction(&mut tx_request, None).await?;

    let raw_tx = tx_request
        .rlp_signed(&liquidator.client_ref().signer().sign_transaction(&tx_request).await?);

    Ok(raw_tx)
}
