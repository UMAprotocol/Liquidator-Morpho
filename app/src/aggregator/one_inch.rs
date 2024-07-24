use eyre::{eyre, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct OneInchClient {
    swap_cli: Arc<Mutex<Client>>,
    chain_id: String,
    last_request_time: Arc<Mutex<Option<std::time::Instant>>>,
    rate_limit_millis: u64,
}

impl OneInchClient {
    pub fn new(api_key: &str, chain_id: &str, rate_limit_millis: u64) -> Self {
        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                headers.insert(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                );
                let mut auth_value = reqwest::header::HeaderValue::from_str(api_key).unwrap();
                auth_value.set_sensitive(true);
                headers.insert(reqwest::header::AUTHORIZATION, auth_value);
                headers
            })
            .build()
            .expect("Failed to create OneInch client");

        Self {
            swap_cli: Arc::new(Mutex::new(client)),
            chain_id: chain_id.to_string(),
            last_request_time: Arc::new(Mutex::new(None)),
            rate_limit_millis,
        }
    }

    async fn rate_limit(&self) {
        let mut last_request_time = self.last_request_time.lock().await;
        if let Some(last_time) = *last_request_time {
            let elapsed = last_time.elapsed();
            if elapsed < Duration::from_millis(self.rate_limit_millis) {
                sleep(Duration::from_millis(self.rate_limit_millis) - elapsed).await;
            }
        }
        *last_request_time = Some(std::time::Instant::now());
    }

    pub async fn quote_token(
        &self,
        token_in: &str,
        token_out: &str,
        in_amt: &str,
    ) -> Result<QuoteResp> {
        self.rate_limit().await;

        let mut query_params = HashMap::new();
        query_params.insert("src", token_in);
        query_params.insert("dst", token_out);
        query_params.insert("amount", in_amt);
        query_params.insert("includeGas", "true");

        let url = format!("https://api.1inch.dev/swap/v6.0/{}/quote", self.chain_id);

        let client = self.swap_cli.lock().await;
        let resp = client.get(&url).query(&query_params).send().await?;

        match resp.status().is_success() {
            true => Ok(resp.json::<QuoteResp>().await?),
            false => Err(eyre!("Error in fetching 1inch quote: {}", resp.text().await?)),
        }
    }

    pub async fn swap_calldata(
        &self,
        token_in: &str,
        token_out: &str,
        from: &str,
        in_amt: &str,
    ) -> Result<SwapCalldataResp> {
        self.rate_limit().await;

        let mut query_params = HashMap::new();
        query_params.insert("src", token_in);
        query_params.insert("dst", token_out);
        query_params.insert("amount", in_amt);
        query_params.insert("from", from);
        query_params.insert("slippage", "10");
        query_params.insert("includeGas", "true");
        query_params.insert("disableEstimate", "true");

        let url = format!("https://api.1inch.dev/swap/v6.0/{}/swap", self.chain_id);

        let client = self.swap_cli.lock().await;
        let resp = client.get(&url).query(&query_params).send().await?;

        match resp.status().is_success() {
            true => Ok(resp.json::<SwapCalldataResp>().await?),
            false => Err(eyre!("Error in fetching 1inch swap calldata: {}", resp.text().await?)),
        }
    }
}

#[derive(Deserialize)]
pub struct QuoteResp {
    #[serde(rename = "dstAmount")]
    pub dst_amount: String,
    pub gas: i32,
}

#[derive(Deserialize)]
pub struct SwapCalldataResp {
    #[serde(rename = "dstAmount")]
    pub dst_amount: String,
    pub tx: Transaction,
}

#[derive(Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub data: String,
    pub value: String,
    pub gas: i32,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
}
