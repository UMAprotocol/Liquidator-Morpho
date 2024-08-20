use ethers::{
    signers::LocalWallet,
    types::{Bytes, U64},
};
use eyre::{eyre, Result};
use jsonrpsee_http_client::{
    transport::Error as HttpError, transport::HttpBackend, HttpClient, HttpClientBuilder,
};
use mev_share_rpc_api::{
    EthBundleApiClient, EthBundleHash, EthSendBundle, FlashbotsSigner, FlashbotsSignerLayer,
};
use tower::{util::MapErr, ServiceBuilder};

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

type BundleClient =
    HttpClient<MapErr<FlashbotsSigner<LocalWallet, HttpBackend>, fn(BoxedError) -> HttpError>>;

pub struct OvalClient {
    bundle_client: BundleClient,
}

fn map_signer_error(e: BoxedError) -> HttpError {
    HttpError::Http(e)
}

impl OvalClient {
    pub fn new(oval_url: &str, oval_signer: LocalWallet) -> Result<Self> {
        // Set up flashbots-style auth middleware
        let signing_middleware = FlashbotsSignerLayer::new(oval_signer);
        let service_builder = ServiceBuilder::new()
            // map signer errors to http errors
            .map_err(map_signer_error as fn(BoxedError) -> HttpError)
            .layer(signing_middleware);

        // Set up the rpc client
        let bundle_client = HttpClientBuilder::default()
            .set_middleware(service_builder)
            .build(oval_url)
            .map_err(|e| eyre!("Failed to build oval client: {}", e))?;

        Ok(Self { bundle_client })
    }

    pub async fn send_raw_txs_bundle(
        &self,
        txs: &Vec<Bytes>,
        block_number: U64,
    ) -> Result<EthBundleHash> {
        let bundle = EthSendBundle { txs: txs.to_owned(), block_number, ..Default::default() };

        Ok(self.bundle_client.send_bundle(bundle).await?)
    }
}
