use alloy::
{
    consensus::
    {
        transaction::
        {
            self, eip4844 as tx_eip4844, TxEnvelope
        }, BlockHeader, Transaction as ConsensusTransaction
    }, 
    eips::BlockNumberOrTag,
    // network::TransactionResponse,
    primitives::
    {
        Address, BlockHash, Bloom, Bytes, Sealable, TxKind, B256, B64, U256
    }, 
    providers::
    {
        Provider, RootProvider
    }, 
    rpc::types::
    {
        error, Block, BlockTransactions, BlockTransactionsKind::Full, Header, Transaction, TransactionReceipt
    }, 
    transports::http::Http
};
use reqwest::Client;
use std::error::Error;

use crate::{provider_info::GenericProvider, transaction_info::TransactionDetails};



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

#[derive(Debug)]
pub struct BlockHeaderData
{
    pub parent_hash: B256,
    pub ommers_hash: B256,
    pub beneficiary: Address,
    pub state_root: B256,
    pub transactions_root: B256,
    pub receipts_root: B256,
    pub logs_bloom: Bloom,
    pub difficulty: U256,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: Bytes,
    pub mix_hash: B256,
    pub nonce: B64,
    pub base_fee_per_gas: Option<u64>,
    pub withdrawals_root: Option<B256>,
    pub blob_gas_used: Option<u64>,
    pub excess_blob_gas: Option<u64>,
    pub parent_beacon_block_root: Option<B256>,
    pub requests_hash: Option<B256>,
}

impl From<Header> for BlockHeaderData
{
    fn from(header: Header) -> Self
    {
        BlockHeaderData
        {
            parent_hash: header.parent_hash,
            ommers_hash: header.ommers_hash,
            beneficiary: header.beneficiary,
            state_root: header.state_root,
            transactions_root: header.transactions_root,
            receipts_root: header.receipts_root,
            logs_bloom: header.logs_bloom,
            difficulty: header.difficulty,
            number: header.number,
            gas_limit: header.gas_limit,
            gas_used: header.gas_used,
            timestamp: header.timestamp,
            extra_data: header.extra_data.clone(),
            mix_hash: header.mix_hash,
            nonce: header.nonce,
            base_fee_per_gas: header.base_fee_per_gas,
            withdrawals_root: header.withdrawals_root,
            blob_gas_used: header.blob_gas_used,
            excess_blob_gas: header.excess_blob_gas,
            parent_beacon_block_root: header.parent_beacon_block_root,
            requests_hash: header.requests_hash
        }
    }
}

#[derive(Debug)]
pub struct BlockTransactionsDetails
{
    pub block_number: u64,
    pub block_hash: B256,
    pub block_timestamp: u64,
    pub base_fee_per_gas: Option<u64>,
    pub transactions: Vec<TransactionDetails>,
}

impl BlockTransactionsDetails
{
    pub async fn build_struct(provider: &GenericProvider, block: Block) -> Result<BlockTransactionsDetails, Box<dyn Error>>
    {
        let returned_struct = BlockTransactionsDetails
        {
            block_number: block.header.number,
            block_hash: block.header.hash,
            block_timestamp: block.header.timestamp,
            base_fee_per_gas: block.header.base_fee_per_gas,
            transactions: Self::build_transactions_vec_from_block_ref(provider, &block).await?,
        };

        Ok(returned_struct) 
    }

    pub async fn build_transactions_vec_from_block_ref(provider: &RootProvider<Http<Client>>, block: &Block) -> Result<Vec<TransactionDetails>, Box<dyn Error>>
    {
        let mut returned_vec: Vec<TransactionDetails> = Vec::new();
        
        if let BlockTransactions::Full(transactions) = &block.transactions
        {
            let base_fee_per_gas = block.header.base_fee_per_gas;

            for transaction in transactions
            {
                let transaction_receipt_option: Option<TransactionReceipt> = provider.get_transaction_receipt(*transaction.inner.tx_hash()).await?;

                if let Some(transaction_receipt) = transaction_receipt_option
                {
                    let returned_transaction_details = TransactionDetails::build(transaction.clone(), transaction_receipt, base_fee_per_gas);

                    returned_vec.push(returned_transaction_details);
                }
            }
        }
        Ok(returned_vec) 
    }
    
    pub async fn build_transactions_vec_from_ident(provider: &RootProvider<Http<Client>>, ident: BlockNumberOrTag) -> Result<Vec<TransactionDetails>, Box<dyn Error>>
    {
        let mut returned_vec: Vec<TransactionDetails> = Vec::new();
        
        if let Some(block) = provider.get_block_by_number(ident, Full).await?
        {
            let mut returned_transaction_details: TransactionDetails;
            let mut transaction_receipt_option: Option<TransactionReceipt>;
            let base_fee_per_gas = block.header.base_fee_per_gas;
            
            if let BlockTransactions::Full(transactions) = block.transactions
            {
                for transaction in transactions
                {
                    transaction_receipt_option = provider.get_transaction_receipt(*transaction.inner.tx_hash()).await?;

                    if let Some(transaction_receipt) = transaction_receipt_option
                    {
                        returned_transaction_details = TransactionDetails::build(transaction, transaction_receipt, base_fee_per_gas);

                        returned_vec.push(returned_transaction_details);
                    }
                }
            }
        }
        Ok(returned_vec)
    }
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

pub async fn build_block_struct(provider: &RootProvider<Http<Client>>, ident: BlockNumberOrTag) -> Result<BlockStruct, Box<dyn Error>>
{  
    let block_option: Option<Block> = provider.get_block_by_number(ident, Full).await?;

    let block = block_option.unwrap();

    let returned_struct = BlockStruct
    {
        block_number: block.header.number,
        hash: block.header.hash.0,
        timestamp: block.header.timestamp,
        parent_hash: block.header.parent_hash.0,
        transactions: block.transactions.hashes().map(|tx| tx.0).collect(),
        gas_used: block.header.gas_used,
        gas_limit: block.header.gas_limit,
        difficulty: block.header.difficulty,
        nonce: block.header.nonce.into(),
        miner: *block.header.beneficiary.0,
        transaction_root: block.header.transactions_root.0,
        state_root: block.header.state_root.0,
        receipts_root: block.header.receipts_root.0,
        logs_bloom: *block.header.logs_bloom.0,
    };

    Ok(returned_struct)
}

pub async fn build_block_struct_simple(provider: &RootProvider<Http<Client>>, ident: BlockNumberOrTag) -> Result<BlockStructSimple, Box<dyn Error>>
{
    let block_data_option = provider.get_block_by_number(ident, Full).await?;
    
    let block_data = block_data_option.unwrap();

    let returned_struct = BlockStructSimple
    {
        block_number: block_data.header.number,
        hash: block_data.header.hash.0,
        timestamp: block_data.header.timestamp,
        parent_hash: block_data.header.parent_hash.0,
    };

    Ok(returned_struct)
}

pub async fn view_block_header_data(provider: &RootProvider<Http<Client>>, ident: BlockNumberOrTag) -> Result<BlockHeaderData, Box<dyn Error>>
{
    let block_data_option: Option<Block> = provider.get_block_by_number(ident, Full).await?;

    // let header_data: BlockHeaderData = block_data_option.header.into();

    if let Some(block_data) = block_data_option 
    {
        let header_data: BlockHeaderData = block_data.header.into();
        Ok(header_data)
    }
    else
    {
        Err("Block not found!".into())
    }
}
