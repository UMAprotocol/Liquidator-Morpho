use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bindings::i_irm::{IIrm, Market, MarketParams};
use ethers::{
    abi::AbiDecode,
    contract::{
        multicall_contract::{Aggregate3Call, Call3},
        MulticallContract, MULTICALL_ADDRESS,
    },
    providers::{Http, Provider},
    types::{Address, U256},
};
use eyre::Result;

use crate::common::math_lib::MathLib;
use crate::db::morpho_db::{MorphoDB, MorphoDBImpl};

pub type BorrowRates = HashMap<U256, U256>;

#[derive(Clone)]
pub struct MarketData {
    pub id: U256,
    pub state: Market,
    pub params: MarketParams,
}

pub trait BorrowRateFetcher {
    fn fetch_borrow_rates(
        &self,
        provider: Arc<Provider<Http>>,
        db: &MorphoDB,
    ) -> impl std::future::Future<Output = Result<BorrowRates>> + Send;
}

impl BorrowRateFetcher for Vec<U256> {
    fn fetch_borrow_rates(
        &self,
        provider: Arc<Provider<Http>>,
        db: &MorphoDB,
    ) -> impl std::future::Future<Output = Result<BorrowRates>> + Send {
        let mut markets = vec![];
        let mut seen_market_ids = HashSet::new();

        for market_id in self.iter() {
            if seen_market_ids.insert(market_id) && db.market_exists(market_id) {
                let market_info = db.get_market(market_id);
                let market_params = db.get_market_params(market_id);

                if market_params.irm != Address::zero() {
                    markets.push(MarketData {
                        id: market_id.to_owned(),
                        state: market_info,
                        params: market_params,
                    });
                }
            }
        }

        fetch_all_borrow_rates(provider, markets.clone())
    }
}

pub trait Accrual {
    fn accrue_interest(&mut self, borrow_rates: &BorrowRates, current_timestamp: &U256);
}

impl Accrual for MarketData {
    // Interest accrual logic from _accrueInterest method in
    // https://github.com/morpho-org/morpho-blue/blob/main/src/Morpho.sol
    fn accrue_interest(&mut self, borrow_rates: &BorrowRates, current_timestamp: &U256) {
        if let Some(borrow_rate) = borrow_rates.get(&self.id) {
            let elapsed = current_timestamp - U256::from(self.state.last_update);
            if elapsed == U256::zero() {
                return;
            }

            let interest = U256::from(self.state.total_borrow_assets)
                .w_mul_down(&borrow_rate.w_taylor_compounded(&elapsed));
            self.state.total_borrow_assets += interest.as_u128();

            // We do not change total_supply_assets, total_supply_shares and fees as this does not affect user health factor.
        }
    }
}

async fn fetch_all_borrow_rates(
    provider: Arc<Provider<Http>>,
    markets: Vec<MarketData>,
) -> Result<BorrowRates> {
    let mut borrow_rates = BorrowRates::new();

    let multicall = MulticallContract::new(MULTICALL_ADDRESS, provider.clone());

    let mut multicall_calls = Aggregate3Call { calls: vec![] };

    for market in markets.iter() {
        let call_data = IIrm::new(Address::random(), provider.clone())
            .borrow_rate_view(market.params.to_owned(), market.state.to_owned())
            .calldata()
            .unwrap();

        multicall_calls.calls.push(Call3 {
            target: market.params.irm,
            call_data,
            allow_failure: true,
        });
    }

    let result = multicall.aggregate_3(multicall_calls.calls).call().await?;

    for (i, value) in result.iter().enumerate() {
        if !value.success {
            continue;
        }

        let values = &value.return_data.0;
        borrow_rates.insert(markets[i].id, AbiDecode::decode(values)?);
    }

    Ok(borrow_rates)
}
