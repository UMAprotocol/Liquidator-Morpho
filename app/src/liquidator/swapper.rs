use crate::aggregator::one_inch::OneInchClient;
use crate::common::constants_lib::*;
use crate::common::math_lib::{MathLib, WAD};
use crate::common::shares_math_lib::SharesMathLib;
use bindings::i_morpho::{Market, MarketParams, Position};
use ethers::prelude::*;
use eyre::Result;
use std::str::FromStr;

pub struct SwapParams {
    pub target: Address,
    pub swap_data: Bytes,
    pub seized_assets: U256,
    pub swapped_debt: U256,
}

pub async fn find_swap_params(
    market_params: &MarketParams,
    position: &Position,
    market: &Market,
    collateral_price: &U256,
    one_inch_client: &OneInchClient,
    liquidator_address: &Address,
) -> Result<SwapParams> {
    let seized_assets = calculate_seized_assets(market_params, position, market, collateral_price);

    let swap_calldata = one_inch_client
        .swap_calldata(
            &market_params.collateral_token.to_string(),
            &market_params.loan_token.to_string(),
            &liquidator_address.to_string(),
            &seized_assets.to_string(),
        )
        .await?;

    let target_address = Address::from_str(&swap_calldata.tx.to)?;
    let swap_data = Bytes::from_str(&swap_calldata.tx.data)?;
    let swapped_debt = U256::from_str(&swap_calldata.dst_amount)?;

    Ok(SwapParams { target: target_address, swap_data, seized_assets, swapped_debt })
}

// This does not handle errors as it is only used in the context of the liquidation and all the shares math replicates
// the logic from Morpho Blue contract.
fn calculate_seized_assets(
    market_params: &MarketParams,
    position: &Position,
    market: &Market,
    collateral_price: &U256,
) -> U256 {
    let repaid_shares = U256::from(position.borrow_shares);
    let total_borrow_assets = U256::from(market.total_borrow_assets);
    let total_borrow_shares = U256::from(market.total_borrow_shares);

    // The liquidation incentive factor is min(maxLiquidationIncentiveFactor, 1/(1 - cursor*(1 - lltv))).
    let liquidation_incentive_factor = MAX_LIQUIDATION_INCENTIVE_FACTOR
        .min(WAD.w_div_down(&(WAD - LIQUIDATION_CURSOR.w_mul_down(&(WAD - market_params.lltv)))));

    repaid_shares
        .to_assets_down(&total_borrow_assets, &total_borrow_shares)
        .w_mul_down(&liquidation_incentive_factor)
        .mul_div_down(&ORACLE_PRICE_SCALE, collateral_price)
}
