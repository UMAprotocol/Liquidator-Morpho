use crate::common::{
    constants_lib::ORACLE_PRICE_SCALE, math_lib::MathLib, shares_math_lib::SharesMathLib,
};
use bindings::i_morpho::{Market, Position};
use ethers::prelude::*;

pub trait HealthCheck {
    fn is_healthy(&self, market: &Market, lltv: &U256, price: &U256) -> bool;
}

impl HealthCheck for Position {
    // Health check logic from _isHealthy method in
    // https://github.com/morpho-org/morpho-blue/blob/main/src/Morpho.sol
    fn is_healthy(&self, market: &Market, lltv: &U256, price: &U256) -> bool {
        let borrowed = U256::from(self.borrow_shares).to_assets_up(
            &U256::from(market.total_borrow_assets),
            &U256::from(market.total_borrow_shares),
        );
        let max_borrow =
            U256::from(self.collateral).mul_div_down(price, &ORACLE_PRICE_SCALE).w_mul_down(lltv);

        max_borrow >= borrowed
    }
}
