use bindings::i_irm::{Market, MarketParams};
use ethers::types::U256;

#[derive(Clone)]
pub struct MarketData {
    pub id: U256,
    pub state: Market,
    pub params: MarketParams,
}
