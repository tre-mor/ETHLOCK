use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
use block_info::get_latest_block_number;
mod block_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let rpc_url = "https://site1.moralis-nodes.com/eth/d2efae9b74dc45bf9c161e4b13c2cd86".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    if let Ok(latest_block_number) = block_info::get_latest_block_number(&provider).await
    {
        println!("latest block number: {}", latest_block_number);

        if let Ok(latest_block) = block_info::build_block_struct(&provider, latest_block_number).await
        {
            println!("latest block: {:#?}", latest_block);
        }

        if let Ok(latest_block) = block_info::build_block_struct_simple(&provider, latest_block_number).await
        {
            println!("latest block: {:#?}", latest_block);
        }
    }
    
    Ok(())
}