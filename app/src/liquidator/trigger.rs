use crate::common::math_lib::MathLib;
use crate::common::shares_math_lib::SharesMathLib;
use crate::one_inch::OneInch;
use bindings::{
    i_morpho::{Market, MarketParams, Position},
    liquidator::{LiquidationParams, Liquidator},
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
    types::{Address, U256},
};
use eyre::{eyre, Result};
use log::{info, warn};
use std::str::FromStr;

use super::swapper::find_swap_params;

const ETH_ADDRESS: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";

pub async fn trigger_liquidation(
    liquidator: &Liquidator<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    user: &Address,
    position: &Position,
    market_params: &MarketParams,
    market: &Market,
    collateral_price: &U256,
    one_inch: &OneInch,
    builder_payment_percent: u8,
) -> Result<()> {
    let swap_params = find_swap_params(
        market_params,
        position,
        market,
        collateral_price,
        one_inch,
        &liquidator.address(),
    )
    .await?;

    let repaid_debt = calculate_repaid_debt(position, market);
    if repaid_debt > swap_params.swapped_debt {
        return Err(eyre!("Repaid debt is greater than swap result from the collateral"));
    }

    let expected_debt_profit = swap_params.swapped_debt - repaid_debt;

    let debt_quote =
        get_price_in_eth(&market_params.loan_token, &expected_debt_profit, one_inch).await?;

    let liquidation_params = LiquidationParams {
        debt_quote,
        builder_payment_percent: U256::from(builder_payment_percent).w_div_down(&U256::from(100)),
        swapper: swap_params.target.to_owned(),
        swap_data: swap_params.swap_data.to_owned(),
    };

    let tx = liquidator
        .liquidate_user(
            market_params.to_owned(),
            user.to_owned(),
            swap_params.seized_assets,
            liquidation_params,
        )
        .send()
        .await?
        .await?;

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

async fn get_price_in_eth(token: &Address, amount: &U256, one_inch: &OneInch) -> Result<U256> {
    if U256::is_zero(amount) {
        return Err(eyre!("Quote input amount is zero"));
    }

    let quote = one_inch.quote_token(&token.to_string(), ETH_ADDRESS, &amount.to_string()).await?;

    let quote_amount = U256::from_str(&quote.dst_amount)?;

    Ok(quote_amount.w_div_down(amount))
}
