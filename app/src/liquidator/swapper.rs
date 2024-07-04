use crate::common::constants_lib::*;
use crate::common::math_lib::{MathLib, WAD};
use crate::common::shares_math_lib::SharesMathLib;
use bindings::i_morpho::{Market, MarketParams, Position};
use ethers::prelude::*;
use eyre::{eyre, Result};

pub struct SwapParams {
    pub target: Address,
    pub swap_data: Bytes,
    pub seized_assets: U256,
}

pub fn find_swap_params(
    market_params: &MarketParams,
    position: &Position,
    market: &Market,
    collateral_price: &U256,
) -> Result<SwapParams> {
    let seized_assets = calculate_seized_assets(market_params, position, market, collateral_price);

    match seized_assets {
        Ok(amount) => {
            // @todo Calculate amount to liquidated
            // Call 1inch/solver APIs to fetch swap path and contract
            let result = SwapParams {
                target: Address::random(),
                swap_data: Bytes::new(),
                seized_assets: amount,
            };
            Ok(result)
        }
        Err(e) => Err(eyre!("Error in calculating seized amount: {}", e)),
    }
}

fn calculate_seized_assets(
    market_params: &MarketParams,
    position: &Position,
    market: &Market,
    collateral_price: &U256,
) -> Result<U256> {
    let repaid_shares = U256::from(position.borrow_shares);
    let total_borrow_assets = U256::from(market.total_borrow_assets);
    let total_borrow_shares = U256::from(market.total_borrow_shares);

    // The liquidation incentive factor is min(maxLiquidationIncentiveFactor, 1/(1 - cursor*(1 - lltv))).
    let liquidation_incentive_factor = MAX_LIQUIDATION_INCENTIVE_FACTOR
        .min(WAD.w_div_down(&(WAD - LIQUIDATION_CURSOR.w_mul_down(&(WAD - market_params.lltv)))));

    let seized_assets = repaid_shares
        .to_assets_down(&total_borrow_assets, &total_borrow_shares)
        .w_mul_down(&liquidation_incentive_factor)
        .mul_div_down(&ORACLE_PRICE_SCALE, collateral_price);

    Ok(seized_assets)
}
