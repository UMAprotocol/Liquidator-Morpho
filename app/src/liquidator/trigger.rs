use crate::aggregator::one_inch::OneInchClient;
use crate::common::math_lib::MathLib;
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
    utils::keccak256,
};
use eyre::{eyre, Result};
use log::{info, warn};
use std::str::FromStr;

use super::swapper::{find_swap_params, SwapParams};

const ETH_ADDRESS: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";

pub async fn trigger_liquidation(
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
) -> Result<()> {
    let swap_params = find_swap_params(
        market_params,
        position,
        market,
        collateral_price,
        one_inch_client,
        &liquidator.address(),
    )
    .await?;

    let repaid_debt = calculate_repaid_debt(position, market);
    if repaid_debt > swap_params.swapped_debt {
        return Err(eyre!("Repaid debt is greater than swap result from the collateral"));
    }

    let expected_debt_profit = swap_params.swapped_debt - repaid_debt;

    let debt_quote =
        get_price_in_eth(&market_params.loan_token, &expected_debt_profit, one_inch_client).await?;

    let (pending_tx, raw_tx) = create_liquidation_tx(
        liquidator,
        market_params,
        user,
        &swap_params,
        &debt_quote,
        builder_payment_percent,
    )
    .await?;

    // Target the next block when sending the bundled raw transaction over the Oval node.
    oval_client.send_raw_txs_bundle(&vec![raw_tx], block_number + 1).await?;

    let tx = pending_tx.await?;

    match tx {
        Some(receipt) => info!("Successful Transaction: {:?}", receipt),
        None => warn!("Empty transaction receipt"),
    }

    Ok(())
}

// This does not handle errors as it is only used in the context of the liquidation and all the shares math replicates
// the logic from Morpho Blue contract.
fn calculate_repaid_debt(position: &Position, market: &Market) -> U256 {
    let borrow_shares = U256::from(position.borrow_shares);
    let total_borrow_assets = U256::from(market.total_borrow_assets);
    let total_borrow_shares = U256::from(market.total_borrow_shares);

    borrow_shares.to_assets_up(&total_borrow_assets, &total_borrow_shares)
}

async fn get_price_in_eth(
    token: &Address,
    amount: &U256,
    one_inch_client: &OneInchClient,
) -> Result<U256> {
    if U256::is_zero(amount) {
        return Err(eyre!("Quote input amount is zero"));
    }

    let quote =
        one_inch_client.quote_token(&token.to_string(), ETH_ADDRESS, &amount.to_string()).await?;

    let quote_amount = U256::from_str(&quote.dst_amount)?;

    Ok(quote_amount.w_div_down(amount))
}

async fn create_liquidation_tx<'a>(
    liquidator: &'a Liquidator<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    market_params: &MarketParams,
    user: &Address,
    swap_params: &SwapParams,
    debt_quote: &U256,
    builder_payment_percent: u8,
) -> Result<(PendingTransaction<'a, Http>, Bytes)> {
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
    liquidator.client().fill_transaction(&mut tx_request, None).await?;

    let raw_tx = tx_request
        .rlp_signed(&liquidator.client_ref().signer().sign_transaction(&tx_request).await?);

    let tx_hash = TxHash(keccak256(raw_tx.clone()));
    let pending_tx = PendingTransaction::new(tx_hash, &liquidator.client_ref().provider());

    Ok((pending_tx, raw_tx))
}
