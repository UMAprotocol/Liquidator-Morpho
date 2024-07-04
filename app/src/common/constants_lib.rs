// Selected constants from https://github.com/morpho-org/morpho-blue/blob/main/src/libraries/ConstantsLib.sol
use ethers::types::U256;

// Oracle price scale.
pub const ORACLE_PRICE_SCALE: U256 = U256([1e36 as u64, 0, 0, 0]);

// Liquidation cursor.
pub const LIQUIDATION_CURSOR: U256 = U256([0.3e18 as u64, 0, 0, 0]);

// Max liquidation incentive factor.
pub const MAX_LIQUIDATION_INCENTIVE_FACTOR: U256 = U256([1.15e18 as u64, 0, 0, 0]);
