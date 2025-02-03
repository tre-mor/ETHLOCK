use alloy::network::Ethereum;
use alloy::providers::{ProviderBuilder, RootProvider};
use alloy::rpc::client::
{
    RpcClient,
    ClientBuilder
};
use alloy::transports::http::Http;
use reqwest::Client;
use url::Url;
use std::error::Error;
use std::time::Duration;

use crate::rpc_client;

pub type GenericProvider = RootProvider<Ethereum>;

pub fn build_provider_from_url_str(url_str: &str) -> Result<GenericProvider, Box<dyn Error>> 
{
    let client = rpc_client::build_rpc_client_from_url_str(url_str)?;

    let provider = RootProvider::new(client);
    
    Ok(provider)
}

pub fn build_provider_from_rpc_client(client: alloy::rpc::client::RpcClient) -> Result<GenericProvider, Box<dyn Error>>
{
    let provider = RootProvider::new(client);
    
    Ok(provider)
}