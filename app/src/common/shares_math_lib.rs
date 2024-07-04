// Helper library to replicate the logic from
// https://github.com/morpho-org/morpho-blue/blob/main/src/libraries/SharesMathLib.sol

use super::math_lib::MathLib;
use ethers::types::U256;

// The number of virtual shares has been chosen low enough to prevent overflows, and high enough to ensure high
// precision computations.
const VIRTUAL_SHARES: U256 = U256([1e6 as u64, 0, 0, 0]);

// A number of virtual assets of 1 enforces a conversion rate between shares and assets when a market is empty.
const VIRTUAL_ASSETS: U256 = U256([1, 0, 0, 0]);

// Shares management library.
pub trait SharesMathLib {
    fn to_shares_down(&self, total_assets: &U256, total_shares: &U256) -> U256;
    fn to_assets_down(&self, total_assets: &U256, total_shares: &U256) -> U256;
    fn to_shares_up(&self, total_assets: &U256, total_shares: &U256) -> U256;
    fn to_assets_up(&self, total_assets: &U256, total_shares: &U256) -> U256;
}

impl SharesMathLib for U256 {
    /// Calculates the value of `assets` quoted in shares, rounding down.
    fn to_shares_down(&self, total_assets: &U256, total_shares: &U256) -> U256 {
        self.mul_div_down(&(total_shares + VIRTUAL_SHARES), &(total_assets + VIRTUAL_ASSETS))
    }

    /// Calculates the value of `shares` quoted in assets, rounding down.
    fn to_assets_down(&self, total_assets: &U256, total_shares: &U256) -> U256 {
        self.mul_div_down(&(total_assets + VIRTUAL_ASSETS), &(total_shares + VIRTUAL_SHARES))
    }

    /// Calculates the value of `assets` quoted in shares, rounding up.
    fn to_shares_up(&self, total_assets: &U256, total_shares: &U256) -> U256 {
        self.mul_div_up(&(total_shares + VIRTUAL_SHARES), &(total_assets + VIRTUAL_ASSETS))
    }

    /// Calculates the value of `shares` quoted in assets, rounding up.
    fn to_assets_up(&self, total_assets: &U256, total_shares: &U256) -> U256 {
        self.mul_div_up(&(total_assets + VIRTUAL_ASSETS), &(total_shares + VIRTUAL_SHARES))
    }
}
