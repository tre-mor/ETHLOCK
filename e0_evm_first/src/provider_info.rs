use alloy::providers::ProviderBuilder;

// alloy::providers::RootProvider<alloy::transports::http::Http<reqwest::Client
pub async fn build_provider_from_url(url: &str) 
-> Result<alloy::providers::RootProvider<alloy::transports::http::Http<reqwest::Client>>, Box<dyn std::error::Error>>
{
    let rpc_url: reqwest::Url = url.parse()?;
    let returned_provider = ProviderBuilder::new().on_http(rpc_url);

    Ok(returned_provider)
}
