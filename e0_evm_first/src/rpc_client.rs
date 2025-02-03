use alloy::rpc::client::
{
    RpcClient,
    ClientBuilder
};
use url::Url;
use std::error::Error;

pub fn build_rpc_client_from_url_str(url_str: &str) -> Result<RpcClient, Box<dyn Error>> 
{
    let url = Url::parse(url_str)?;

    let client = ClientBuilder::default().http(url);

    Ok(client)
}