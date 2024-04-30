use std::sync::Arc;
use std::time::Duration;
use ethers::providers::{Http, Provider, RetryClient, RetryClientBuilder};
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum RetryClientError {
    #[error("http provider cant be created for {0}: {1}")]
    HttpProviderCantBeCreated(String, String),
}

pub fn create_retry_client(
    rpc_url: &str,
) -> Result<Arc<Provider<RetryClient<Http>>>, RetryClientError> {
    let url = Url::parse(rpc_url).map_err(|e| {
        RetryClientError::HttpProviderCantBeCreated(rpc_url.to_string(), e.to_string())
    })?;
    let provider = Http::new(url);
    // gone quite intense with retries for now incase a node is down
    Ok(Arc::new(Provider::new(
        RetryClientBuilder::default()
            .rate_limit_retries(50)
            .timeout_retries(10)
            .initial_backoff(Duration::from_millis(500))
            .build(
                provider,
                Box::<ethers::providers::HttpRateLimitRetryPolicy>::default(),
            ),
    )))
}