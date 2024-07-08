use crate::one_inch::OneInch;
use bindings::{
    i_morpho::{Market, MarketParams, Position},
    liquidator::Liquidator,
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
    types::{Address, U256},
};
use eyre::Result;
use log::{info, warn};

use super::swapper::find_swap_params;

pub async fn trigger_liquidation(
    liquidator: &Liquidator<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    user: &Address,
    position: &Position,
    market_params: &MarketParams,
    market: &Market,
    collateral_price: &U256,
    one_inch: &OneInch,
) -> Result<()> {
    let swap_params = find_swap_params(
        market_params,
        position,
        market,
        collateral_price,
        one_inch,
        &liquidator.address(),
    )
    .await?;

    let tx = liquidator
        .liquidate_user(
            market_params.to_owned(),
            user.to_owned(),
            swap_params.target,
            swap_params.seized_assets,
            swap_params.swap_data,
        )
        .send()
        .await?
        .await?;

    match tx {
        Some(receipt) => info!("Successful Transaction: {:?}", receipt),
        None => warn!("Empty transaction receipt"),
    }

    Ok(())
}
