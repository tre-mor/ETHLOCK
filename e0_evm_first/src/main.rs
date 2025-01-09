use alloy::primitives::U256;
use alloy::providers::{Provider, ProviderBuilder};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let rpc_url = "https://site1.moralis-nodes.com/eth/d2efae9b74dc45bf9c161e4b13c2cd86".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {}", latest_block);
    
    Ok(())
}
