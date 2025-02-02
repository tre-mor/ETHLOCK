use alloy::providers::RootProvider;
use alloy::rpc::client::RpcClient;
use alloy::transports::http::Http;
use reqwest::Client;
use url::Url;
use std::error::Error;

// pub type GenericProvider = RootProvider<Http<Client>>;

// pub fn build_provider_from_url(url_str: &str) -> Result<GenericProvider, Box<dyn Error>> {
//     let url = Url::parse(url_str)?;
//     let transport = Http::new(url)?;
//     let provider = RootProvider::new(transport);
//     Ok(provider)
// }

// pub type GenericProvider = RpcClient;

pub fn build_provider_from_url(url_str: &str) -> Result<RpcClient, Box<dyn Error>> 
{
    let url = Url::parse(url_str)?;
    let provider = RpcClient::new_http(url);
    Ok(provider)
}