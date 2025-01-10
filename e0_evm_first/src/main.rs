use alloy::{
    eips::BlockNumberOrTag::{ Latest, Finalized, Safe, Earliest, Pending, Number },
    primitives::U256,
    providers::ProviderBuilder,
};
use block_info::view_block_header_data;
// use block_info::get_latest_block_number;
mod block_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let rpc_url = "https://site1.moralis-nodes.com/eth/d2efae9b74dc45bf9c161e4b13c2cd86".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    
    // if let Ok(latest_block_number) = block_info::get_latest_block_number(&provider).await
    // {
    //     println!("latest block number: {}", latest_block_number);

    //     // if let Ok(latest_block) = block_info::build_block_struct(&provider, latest_block_number).await
    //     // {
    //     //     println!("latest block: {:#?}", latest_block);
    //     // }

    //     if let Ok(latest_block) = block_info::build_block_struct_simple(&provider, Number(latest_block_number)).await
    //     {
    //         println!("latest block: {:#?}", latest_block);
    //     }
    // }

    // if let Ok(latest_block_number) = block_info::get_latest_block_number(&provider).await
    // {
    //     println!("latest block number: {}", latest_block_number);

    //     if let Ok(latest_n_block_numbers) = block_info::get_latest_n_block_numbers(latest_block_number, 5).await
    //     {
    //         println!("previous {} blocks: {:#?}", latest_n_block_numbers.len(), latest_n_block_numbers);
    //     }
    // }
    
    println!("{:?}", view_block_header_data(&provider, Latest).await);
    
    Ok(())
}