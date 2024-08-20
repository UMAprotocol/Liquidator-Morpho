use app::{
    aggregator::one_inch::OneInchClient,
    common::types::MarketData,
    db::{
        event_processor::ProcessEvent,
        health_checker::HealthCheck,
        morpho_db::{FileManager, MorphoDB, MorphoDBImpl},
    },
    interest::{Accrual, BorrowRateFetcher},
    liquidator::trigger::trigger_liquidation,
    oracles::price_fetcher::PriceFetcher,
    oval::oval_client,
};
use bindings::{
    i_morpho::{
        AccrueInterestFilter, BorrowFilter, CreateMarketFilter, LiquidateFilter, RepayFilter,
        SupplyCollateralFilter, SupplyFilter, WithdrawCollateralFilter, WithdrawFilter,
    },
    liquidator::Liquidator,
};
use dotenv::dotenv;
use ethers::{
    abi::RawLog,
    prelude::{Middleware, *},
    providers::{Http, Provider, ProviderExt, StreamExt, Ws},
    types::{Address, U64},
};
use eyre::{eyre, Result};
use log::{error, info, warn};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    let config = Config::build()?;

    let provider = Provider::<Ws>::connect(config.wss_rpc_url.to_owned()).await?;
    let http_provider = Provider::<Http>::try_connect(&config.http_rpc_url).await?;

    let client = Arc::new(provider);

    let http_client = Arc::new(http_provider);

    let chain_id = http_client.get_chainid().await?;
    let one_inch_client = OneInchClient::new(
        &config.one_inch_api_key,
        &config.one_inch_base_url,
        &chain_id.to_string(),
        config.one_inch_rate_limit,
    );

    let mut db: MorphoDB = MorphoDB::load_memory_db(&config.file_name)?;

    let last_block =
        sync_to_lastest_block(&config, &mut db, http_client.clone(), None, None).await?;
    info!("Last block: {last_block}");

    let result =
        subscribe(&config, &client, &mut db, &http_client, &one_inch_client, &chain_id).await;

    match result {
        Ok(_) => info!("Listening completed"),
        Err(err) => error!("Error in listening: {}", err),
    }

    Ok(())
}

async fn subscribe(
    config: &Config,
    ws_client: &Arc<Provider<Ws>>,
    db: &mut MorphoDB,
    client: &Arc<Provider<Http>>,
    one_inch_client: &OneInchClient,
    chain_id: &U256,
) -> Result<()> {
    let mut new_block_stream = ws_client.subscribe_blocks().await?;

    info!("Started listening to new blocks");

    loop {
        tokio::select! {
            block = new_block_stream.next() => {
                let block = if let Some(block) = block { block } else { continue };
                let block_number = if let Some(number) = block.number { number } else { continue };

                info!("New block received: {:?}", block_number);

                match process_new_block(
                    config,
                    db,
                    client,
                    one_inch_client,
                    &block_number,
                    &block.timestamp,
                    chain_id.as_u64(),
                ).await {
                    Ok(_) => info!("Successfully processed block"),
                    Err(e) => error!("Error while processing block: {:?}", e),
                }
            }
            else => break
        };
    }

    Ok(())
}

async fn process_new_block(
    config: &Config,
    db: &mut MorphoDB,
    client: &Arc<Provider<Http>>,
    one_inch_client: &OneInchClient,
    block_number: &U64,
    block_timestamp: &U256,
    chain_id: u64,
) -> Result<()> {
    sync_to_lastest_block(config, db, client.clone(), Some(block_number), Some(block_timestamp))
        .await?;

    let oracle_prices = db
        .get_all_markets()
        .fetch_prices(client.clone(), config.unlocked_oval_oracle_address.parse::<Address>()?)
        .await?;

    let market_ids = db.get_all_market_ids();

    let borrow_rates = market_ids.fetch_borrow_rates(client.clone(), db).await?;

    let wallet: LocalWallet =
        get_from_config("PRIVATE_KEY".to_string())?.parse::<LocalWallet>()?.with_chain_id(chain_id);

    let http_client = Arc::new(SignerMiddleware::new(
        Provider::<Http>::try_connect(&config.http_rpc_url).await?,
        wallet,
    ));

    let bundle_signer = get_from_config("BUNDLE_SIGNER_KEY".to_string())?.parse::<LocalWallet>()?;

    let oval_client = Arc::new(oval_client::OvalClient::new(&config.oval_rpc_url, bundle_signer)?);

    let liquidator = Liquidator::new(config.liquidator_address.parse::<Address>()?, http_client);

    for market_id in market_ids {
        let mut market = MarketData {
            id: market_id,
            state: db.get_market(&market_id),
            params: db.get_market_params(&market_id),
        };
        market.accrue_interest(&borrow_rates, block_timestamp);

        let users = db.get_all_users(&market_id);
        let price = oracle_prices.get(&market.params.oracle);

        if let Some(price) = price {
            if price.is_zero() {
                continue;
            }

            for user in users {
                let position = db.get_position(&market_id, &user);
                if !position.is_healthy(&market.state, &market.params.lltv, price) {
                    warn!(
                        "Position can be liquidated for {:?} and market: {market_id:#032x}",
                        &user
                    );
                    info!("Triggering liquidation");

                    let result = trigger_liquidation(
                        client.clone(),
                        &liquidator,
                        &user,
                        &position,
                        &market.params,
                        &market.state,
                        price,
                        one_inch_client,
                        config.builder_payment_percent,
                        &oval_client,
                        block_number,
                        chain_id,
                    )
                    .await;
                    match result {
                        Ok(tx_hash) => info!("Submitted liquidation with tx hash: {:?}", tx_hash),
                        Err(e) => error!("Errored while liquidation: {}", e),
                    }
                }
            }
        } else {
            continue;
        };
    }

    Ok(())
}

async fn get_block_timestamp(client: Arc<Provider<Http>>, block_number: u64) -> Result<u128> {
    let block = client.get_block(block_number).await?;
    match block {
        Some(block) => Ok(block.timestamp.as_u128()),
        None => Err(eyre!("Block {} not found", block_number)),
    }
}

async fn sync_to_lastest_block(
    config: &Config,
    db: &mut MorphoDB,
    client: Arc<Provider<Http>>,
    current_block: Option<&U64>,
    current_timestamp: Option<&U256>,
) -> Result<u64> {
    let current_block = match current_block {
        Some(block_number) => block_number.to_owned(),
        None => client.get_block_number().await?,
    }
    .as_u64();

    let current_timestamp = match current_timestamp {
        Some(timestamp) => timestamp.as_u128(),
        None => get_block_timestamp(client.clone(), current_block).await?,
    };

    let mut start_block = if db.last_block_sync < config.block_start {
        config.block_start
    } else {
        db.last_block_sync + 1
    };

    let mut logs: Vec<Log> = Vec::new();

    loop {
        let mut end_block = start_block + 99_000;

        if end_block > current_block {
            end_block = current_block;
        }

        info!("Syncing for blocks between {} and {}", start_block, end_block);

        let filter = Filter::new()
            .address(config.morpho_address.parse::<Address>()?)
            .from_block(start_block)
            .to_block(end_block);

        let mut morpho_logs = client.get_logs(&filter).await?;

        logs.append(&mut morpho_logs);
        start_block += 99_000;
        if start_block > current_block {
            break;
        }
    }

    for log in logs.iter() {
        if log.topics.is_empty() {
            continue;
        }

        let log_timestamp = match log.block_number {
            None => Err(eyre!("Block number not found in log"))?,
            Some(block_number) => match block_number.as_u64() == current_block {
                true => current_timestamp,
                false => get_block_timestamp(client.clone(), block_number.as_u64()).await?,
            },
        };

        match log.topics[0] {
            x if x == CreateMarketFilter::signature() => {
                let event = <CreateMarketFilter as EthLogDecode>::decode_log(&RawLog::from(
                    log.to_owned(),
                ))?;

                if event.market_params.lltv.is_zero() {
                    continue;
                }
                event.process(db, log_timestamp);
            }
            x if x == BorrowFilter::signature() => {
                let event =
                    <BorrowFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;
                event.process(db, log_timestamp);
            }
            x if x == SupplyCollateralFilter::signature() => {
                let event = <SupplyCollateralFilter as EthLogDecode>::decode_log(&RawLog::from(
                    log.to_owned(),
                ))?;
                event.process(db, log_timestamp);
            }
            x if x == RepayFilter::signature() => {
                let event =
                    <RepayFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;
                event.process(db, log_timestamp);
            }
            x if x == WithdrawCollateralFilter::signature() => {
                let event = <WithdrawCollateralFilter as EthLogDecode>::decode_log(&RawLog::from(
                    log.to_owned(),
                ))?;
                event.process(db, log_timestamp);
            }
            x if x == LiquidateFilter::signature() => {
                let event =
                    <LiquidateFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;

                event.process(db, log_timestamp);
            }
            x if x == SupplyFilter::signature() => {
                let event =
                    <SupplyFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;

                event.process(db, log_timestamp);
            }
            x if x == WithdrawFilter::signature() => {
                let event =
                    <WithdrawFilter as EthLogDecode>::decode_log(&RawLog::from(log.to_owned()))?;

                event.process(db, log_timestamp);
            }
            x if x == AccrueInterestFilter::signature() => {
                let event = <AccrueInterestFilter as EthLogDecode>::decode_log(&RawLog::from(
                    log.to_owned(),
                ))?;

                event.process(db, log_timestamp);
            }
            _ => {}
        }
    }

    db.last_block_sync = current_block;

    db.write_memory_db(&config.file_name)?;

    Ok(current_block)
}

struct Config {
    wss_rpc_url: String,
    http_rpc_url: String,
    oval_rpc_url: String,
    file_name: String,
    morpho_address: String,
    block_start: u64,
    liquidator_address: String,
    unlocked_oval_oracle_address: String,
    one_inch_base_url: String,
    one_inch_api_key: String,
    one_inch_rate_limit: u64,
    builder_payment_percent: u8,
}

impl Config {
    fn build() -> Result<Self> {
        let builder_payment_percent = get_from_config_optional(
            "BUILDER_PAYMENT_PERCENT".to_string(),
            Some("90".to_string()),
        )?
        .parse::<u8>()?;
        if builder_payment_percent > 100 {
            return Err(eyre!("Builder payment percent cannot be greater than 100"));
        }

        Ok(Config {
            wss_rpc_url: get_from_config("WSS_RPC_URL".to_string())?,
            http_rpc_url: get_from_config("HTTP_RPC_URL".to_string())?,
            oval_rpc_url: get_from_config_optional(
                "OVAL_RPC_URL".to_string(),
                Some("https://rpc.oval.xyz:443".to_string()),
            )?,
            file_name: get_from_config("FILE_NAME".to_string())?,
            morpho_address: get_from_config_optional(
                "MORPHO_ADDRESS".to_string(),
                Some("0xBBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb".to_string()),
            )?,
            block_start: get_from_config_optional(
                "BLOCK_START".to_string(),
                Some("18883124".to_string()),
            )?
            .parse::<u64>()?,
            liquidator_address: get_from_config("LIQUIDATOR_ADDRESS".to_string())?,
            unlocked_oval_oracle_address: get_from_config(
                "UNLOCKED_OVAL_ORACLE_ADDRESS".to_string(),
            )?,
            one_inch_base_url: get_from_config_optional(
                "ONE_INCH_BASE_URL".to_string(),
                Some("https://api.1inch.dev".to_string()),
            )?,
            one_inch_api_key: get_from_config("ONE_INCH_API_KEY".to_string())?,
            one_inch_rate_limit: get_from_config_optional(
                "ONE_INCH_RATE_LIMIT".to_string(),
                Some("1200".to_string()),
            )?
            .parse::<u64>()?,
            builder_payment_percent,
        })
    }
}

fn get_from_config(key: String) -> Result<String> {
    Ok(std::env::var(key)?)
}

fn get_from_config_optional(key: String, default_value: Option<String>) -> Result<String> {
    match std::env::var(&key) {
        Ok(value) => Ok(value),
        Err(_) => match default_value {
            Some(default) => Ok(default),
            None => Err(eyre::eyre!(
                "Environment variable {} not found and no default value provided",
                key
            )),
        },
    }
}
