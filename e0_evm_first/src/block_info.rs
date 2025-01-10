use alloy::
{
    consensus::BlockHeader,
    eips::BlockNumberOrTag,
    primitives::
    {
        Address, BlockHash, Bloom, Bytes, Sealable, B256, B64, U256,
    },
    providers::
    {
        Provider, RootProvider
    },
    rpc::types::
    {
        Block, BlockTransactionsKind
    },
    transports::http::Http,
};
use reqwest::Client;
use std::error::Error;

#[derive(Debug)]
pub struct BlockStruct
{
    block_number: u64,
    hash: [u8; 32],
    timestamp: u64,
    parent_hash: [u8; 32],
    transactions: Vec<[u8; 32]>,
    gas_used: u64,
    gas_limit: u64,
    difficulty: U256,
    nonce: u64,
    miner: [u8; 20],
    transaction_root: [u8; 32],
    state_root: [u8; 32],
    receipts_root: [u8; 32],
    logs_bloom: [u8; 256],
}

#[derive(Debug)]
pub struct BlockStructSimple
{
    block_number: u64,
    hash: [u8; 32],
    timestamp: u64,
    parent_hash: [u8; 32],
}

pub async fn get_latest_block_number(provider: &RootProvider<Http<Client>>) -> Result<u64, Box<dyn Error>>
{
    let latest_block_number = provider.get_block_number().await?;

    Ok(latest_block_number)
}

pub async fn get_latest_n_block_numbers(latest_block_number: u64, n: u64) -> Result<Vec<u64>, Box<dyn Error>>
{
    let block_numbers_to_collect = n - 1;
    let returned_vec: Vec<u64> = (1..=block_numbers_to_collect).map(|i| latest_block_number - i).collect();
    
    Ok(returned_vec)
}

pub async fn build_block_struct(provider: &RootProvider<Http<Client>>, number_or_tag: BlockNumberOrTag) -> Result<BlockStruct, Box<dyn Error>>

{  
    // let parsed_block_number: BlockNumberOrTag = BlockNumberOrTag::Number(block_number);
    let block_data: Option<Block> = provider.get_block_by_number(number_or_tag, BlockTransactionsKind::Full).await?;

    let block_data_unwrapped = block_data.unwrap();

    let returned_struct = BlockStruct
    {
        block_number: block_data_unwrapped.header.number,
        hash: block_data_unwrapped.header.hash.0,
        timestamp: block_data_unwrapped.header.timestamp,
        parent_hash: block_data_unwrapped.header.parent_hash.0,
        transactions: block_data_unwrapped.transactions.hashes().map(|tx| tx.0).collect(),
        gas_used: block_data_unwrapped.header.gas_used,
        gas_limit: block_data_unwrapped.header.gas_limit,
        difficulty: block_data_unwrapped.header.difficulty,
        nonce: block_data_unwrapped.header.nonce.into(),
        miner: *block_data_unwrapped.header.beneficiary.0,
        transaction_root: block_data_unwrapped.header.transactions_root.0,
        state_root: block_data_unwrapped.header.state_root.0,
        receipts_root: block_data_unwrapped.header.receipts_root.0,
        logs_bloom: *block_data_unwrapped.header.logs_bloom.0,
    };

    Ok(returned_struct)
}

pub async fn build_block_struct_simple(provider: &RootProvider<Http<Client>>, number_or_tag: BlockNumberOrTag) -> Result<BlockStructSimple, Box<dyn Error>>
{
    // let parsed_block_number: BlockNumberOrTag = BlockNumberOrTag::Number(block_number);
    // let parsed_block_number: BlockNumberOrTag = BlockNumberOrTag::Latest;

    // println!("parsed number {}", parsed_block_number);

    let block_data = provider.get_block_by_number(number_or_tag, BlockTransactionsKind::Full).await?;
    
    let block_data_unwrapped = block_data.unwrap();

    let returned_struct = BlockStructSimple
    {
        block_number: block_data_unwrapped.header.number,
        hash: block_data_unwrapped.header.hash.0,
        timestamp: block_data_unwrapped.header.timestamp,
        parent_hash: block_data_unwrapped.header.parent_hash.0,
    };

    Ok(returned_struct)
}

// pub async fn view_block_header_data(provider: &RootProvider<Http<Client>>, block_number: u64) -> Result<BlockHeader, Box<dyn Error>>
// {
//     let parsed_block_number: BlockNumberOrTag = BlockNumberOrTag::Number(block_number);
// }