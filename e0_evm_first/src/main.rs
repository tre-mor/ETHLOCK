use std::str::FromStr;

use alloy::{
    eips::BlockNumberOrTag::{ Earliest, Finalized, Latest, Number, Pending, Safe },
    primitives::{B256, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::BlockTransactionsKind::Full,
};
use block_info::{view_block_header_data, TransactionDetails};
// use block_info::get_latest_block_number;
mod block_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let rpc_url = "https://site1.moralis-nodes.com/eth/d2efae9b74dc45bf9c161e4b13c2cd86".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // let block_option = provider.get_block_by_number(Latest, Full).await?;

    // if let Some(block) = block_option
    // {
    //     let btxdet: block_info::BlockTransactionsDetails = block_info::BlockTransactionsDetails::build_struct(&provider, block).await?;

    //     for transaction in btxdet.transactions
    //     {
    //         transaction.print_details();
    //     }
    // }

    let tx = TransactionDetails::get(&provider, B256::from_str("0x4a47971ee2e5e3cb56bc601f1475a51d2993a640faf63afbf1b08c58d00136b9").unwrap()).await;

    if let Ok(tx) = tx
    {
        tx.print_details();
    }
    
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
    
    // println!("{:?}", view_block_header_data(&provider, Latest).await);
    
    Ok(())
}

/* 
I am building an Ethereum block explorer with Rust, alloy.rs (a rust crate for interfacing with the Ethereum blockchain), Tauri, and Swift. The block explorer will display block number, hash, timestamp, and transaction details. It will also implement basic filtering. Remember to refer to the documentation (the entire repo in the alloy directory and the alloy_rs_book) when writing the code. Prioritize efficiency and clarity. All code must be well explained with comments on every line.
 */