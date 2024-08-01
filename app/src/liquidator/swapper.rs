use crate::aggregator::one_inch::OneInchClient;
use bindings::i_morpho::MarketParams;
use ethers::{prelude::*, utils::hex::ToHexExt};
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
    seized_assets: U256,
    one_inch_client: &OneInchClient,
    liquidator_address: &Address,
) -> Result<SwapParams> {
    let token_in = market_params.collateral_token.encode_hex_with_prefix();
    let token_out = market_params.loan_token.encode_hex_with_prefix();
    let from = liquidator_address.encode_hex_with_prefix();

    let swap_calldata = one_inch_client
        .swap_calldata(&token_in, &token_out, &from, &seized_assets.to_string())
        .await?;

    let target_address = Address::from_str(&swap_calldata.tx.to)?;
    let swap_data = Bytes::from_str(&swap_calldata.tx.data)?;
    let swapped_debt = U256::from(swap_calldata.dst_amount.parse::<u128>()?);

    Ok(SwapParams { target: target_address, swap_data, seized_assets, swapped_debt })
}
