use alloy::rpc::client::
{
    RpcClient,
    ClientBuilder
};
use url::Url;
use std::error::Error;

/// Builds an RpcClient from a URL string
///
/// The string should be a valid URL that the RpcClient can use to connect to the Ethereum node.
///
/// Returns an error if the URL is not a valid URL.
pub fn build_rpc_client_from_url_str(url_str: &str) -> Result<RpcClient, Box<dyn Error>> 
{
    let url = Url::parse(url_str)?;

    let client = ClientBuilder::default().http(url);

    Ok(client)
}