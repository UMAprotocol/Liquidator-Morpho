// Selected constants from https://github.com/morpho-org/morpho-blue/blob/main/src/libraries/ConstantsLib.sol
use ethers::types::U256;

// Oracle price scale. As 1e36 does not fit in a u64 and we cannot call `U256::from` in const context, we have to
// set it manually here.
pub const ORACLE_PRICE_SCALE: U256 =
    U256([0xb34b9f1000000000 as u64, 0xc097ce7bc90715 as u64, 0, 0]);

// Liquidation cursor.
pub const LIQUIDATION_CURSOR: U256 = U256([0.3e18 as u64, 0, 0, 0]);

// Max liquidation incentive factor.
pub const MAX_LIQUIDATION_INCENTIVE_FACTOR: U256 = U256([1.15e18 as u64, 0, 0, 0]);
