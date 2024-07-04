// Helper library to replicate the logic from
// https://github.com/morpho-org/morpho-blue/blob/main/src/libraries/MathLib.sol
use ethers::types::U256;

pub const WAD: U256 = U256([1e18 as u64, 0, 0, 0]);

// Library to manage fixed-point arithmetic.
pub trait MathLib {
    fn w_mul_down(&self, y: &U256) -> U256;
    fn w_div_down(&self, y: &U256) -> U256;
    fn w_div_up(&self, y: &U256) -> U256;
    fn mul_div_down(&self, y: &U256, d: &U256) -> U256;
    fn mul_div_up(&self, y: &U256, d: &U256) -> U256;
    fn w_taylor_compounded(&self, n: &U256) -> U256;
}

impl MathLib for U256 {
    /// Returns (`x` * `y`) / `WAD` rounded down.
    fn w_mul_down(&self, y: &U256) -> U256 {
        self.mul_div_down(y, &WAD)
    }

    /// Returns (`x` * `WAD`) / `y` rounded down.
    fn w_div_down(&self, y: &U256) -> U256 {
        self.mul_div_down(&WAD, y)
    }

    /// Returns (`x` * `WAD`) / `y` rounded up.
    fn w_div_up(&self, y: &U256) -> U256 {
        self.mul_div_up(&WAD, y)
    }

    /// Returns (`x` * `y`) / `d` rounded down.
    fn mul_div_down(&self, y: &U256, d: &U256) -> U256 {
        self * y / d
    }

    /// Returns (`x` * `y`) / `d` rounded up.
    fn mul_div_up(&self, y: &U256, d: &U256) -> U256 {
        (self * y) + (d - 1) / d
    }

    /// Returns the sum of the first three non-zero terms of a Taylor expansion of e^(nx) - 1, to approximate a
    /// continuous compound interest rate.
    fn w_taylor_compounded(&self, n: &U256) -> U256 {
        let first_term = self * n;
        let second_term = first_term.mul_div_down(&first_term, &(U256::from(2) * WAD));
        let third_term = second_term.mul_div_down(&first_term, &(U256::from(3) * WAD));

        first_term + second_term + third_term
    }
}
