use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
mod block_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let rpc_url = "https://site1.moralis-nodes.com/eth/d2efae9b74dc45bf9c161e4b13c2cd86".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    if let Ok(latest_block_number) = block_info::get_latest_block_number(provider).await
    {
        println!("latest block number: {}", latest_block_number);
    }
    
    Ok(())
}
