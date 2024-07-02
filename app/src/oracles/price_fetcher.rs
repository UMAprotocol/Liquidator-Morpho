use std::{collections::HashMap, sync::Arc};

use bindings::{i_irm::MarketParams, unlocked_oval_oracle::UnlockedOvalOracle};
use ethers::{
    abi::AbiDecode,
    contract::{
        multicall_contract::{Aggregate3Call, Call3},
        MulticallContract, MULTICALL_ADDRESS,
    },
    core::types::Address,
    providers::{Http, Provider},
    types::U256,
};
use eyre::Result;

pub type OraclePrices = HashMap<Address, U256>;

pub trait PriceFetcher {
    fn fetch_prices(
        &self,
        provider: Arc<Provider<Http>>,
        unlocked_oval_oracle_address: Address,
    ) -> impl std::future::Future<Output = Result<OraclePrices>> + Send;
}

impl PriceFetcher for Vec<MarketParams> {
    fn fetch_prices(
        &self,
        provider: Arc<Provider<Http>>,
        unlocked_oval_oracle_address: Address,
    ) -> impl std::future::Future<Output = Result<OraclePrices>> + Send {
        let mut oracle_addresses = Vec::new();
        self.iter().for_each(|market_param| {
            if !oracle_addresses.contains(&market_param.oracle) {
                oracle_addresses.push(market_param.oracle);
            }
        });

        fetch_all_oracle_prices(provider, oracle_addresses, unlocked_oval_oracle_address)
    }
}

pub async fn fetch_all_oracle_prices(
    provider: Arc<Provider<Http>>,
    oracles_addresses: Vec<Address>,
    unlocked_oval_oracle_address: Address,
) -> Result<OraclePrices> {
    let mut prices: OraclePrices = OraclePrices::new();

    let multicall = MulticallContract::new(MULTICALL_ADDRESS, provider.clone());

    let mut multicall_calls: Aggregate3Call = Aggregate3Call { calls: vec![] };

    for oracle_address in oracles_addresses {
        let calldata = UnlockedOvalOracle::new(Address::random(), provider.clone())
            .unlocked_oracle_price(oracle_address)
            .calldata()
            .unwrap();

        multicall_calls.calls.push(Call3 {
            target: unlocked_oval_oracle_address,
            call_data: calldata.to_owned(),
            allow_failure: true,
        });
    }

    let result = multicall.aggregate_3(multicall_calls.calls.clone()).call().await?;

    for (i, value) in result.iter().enumerate() {
        if !value.success {
            continue;
        }

        let values = &value.return_data.0;
        prices.insert(multicall_calls.calls[i].target, AbiDecode::decode(values)?);
    }

    Ok(prices)
}
