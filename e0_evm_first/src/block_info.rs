use alloy::{providers::{Provider, RootProvider}, transports::http::Http};
use reqwest::Client;


pub async fn get_latest_block_number(provider: RootProvider<Http<Client>>) -> Result<u64, Box<dyn std::error::Error>>
{
    let latest_block_number = provider.get_block_number().await?;

    Ok(latest_block_number)
}