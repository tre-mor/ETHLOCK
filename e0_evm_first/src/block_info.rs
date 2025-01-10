use alloy::
{
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

pub async fn build_block_struct(provider: &RootProvider<Http<Client>>, block_number: u64) -> Result<BlockStruct, Box<dyn Error>>

{
    // let block_test;
    
    let parsed_block_number: BlockNumberOrTag = BlockNumberOrTag::Number(block_number);
    let block_data: Option<Block> = provider.get_block_by_number(parsed_block_number, BlockTransactionsKind::Full).await?;

    // if let Some(block_field_data) = block_data
    // {
    //     let BlockStruct
    //     {
    //         block_number: block.header.number,
    //         timestamp: block.header.timestamp,
    //         hash: block.header.hash.0,
    //         parent_hash: block.header.parent_hash.0,
    //         gas_used: block.header.gas_used,
    //         gas_limit,
    //         difficulty,
    //         nonce,
    //         miner,
    //         transaction_root,
    //         state_root,
    //         receipts_root,
    //         logs_bloom,
    //     } = block_field_data;
    // }

    // if let Some(block_field_data) = block_data
    // {
    //     block_test = block_field_data;
    // }

    let block_test = block_data.unwrap();

    let temp_return = BlockStruct
    {
        block_number: block_test.header.number,
        hash: block_test.header.hash.0,
        timestamp: block_test.header.timestamp,
        parent_hash: block_test.header.parent_hash.0,
        transactions: block_test.transactions.hashes().map(|tx| tx.0).collect(),
        gas_used: block_test.header.gas_used,
        gas_limit: block_test.header.gas_limit,
        difficulty: block_test.header.difficulty,
        nonce: block_test.header.nonce.into(),
        miner: *block_test.header.beneficiary.0,
        transaction_root: block_test.header.transactions_root.0,
        state_root: block_test.header.state_root.0,
        receipts_root: block_test.header.receipts_root.0,
        logs_bloom: *block_test.header.logs_bloom.0,
    };

    Ok(temp_return)
}

pub async fn build_block_struct_simple(provider: &RootProvider<Http<Client>>, block_number: u64) -> Result<BlockStructSimple, Box<dyn Error>>
{
    let parsed_block_number: BlockNumberOrTag = BlockNumberOrTag::Number(block_number);
    let block_data = provider.get_block_by_number(parsed_block_number, BlockTransactionsKind::Full).await?;

    
    let block_test = block_data.unwrap();

    let temp_return = BlockStructSimple
    {
        block_number: block_test.header.number,
        hash: block_test.header.hash.0,
        timestamp: block_test.header.timestamp,
        parent_hash: block_test.header.parent_hash.0,
    };

    Ok(temp_return)
}