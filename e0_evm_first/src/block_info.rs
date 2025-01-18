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
    pub async fn build_struct(provider: &RootProvider<Http<Client>>, block: Block) -> Result<BlockTransactionsDetails, Box<dyn Error>>
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

#[derive(Debug)]
pub struct TransactionDetails
{
    submission_details: SubmissionDetails,
    outcome_details: OutcomeDetails,
}

impl TransactionDetails
{
    pub async fn get(provider: &RootProvider<Http<Client>>, transaction_hash: B256) -> Result<TransactionDetails, Box<dyn Error>>
    {
        if let Ok(Some(transaction)) = provider.get_transaction_by_hash(transaction_hash).await
        {
            if let Ok(Some(transaction_receipt)) = provider.get_transaction_receipt(transaction_hash).await
            {
                let mut base_fee_per_gas = None;

                if let Some(block_hash) = transaction_receipt.block_hash
                {
                    if let Ok(Some(block)) = provider.get_block_by_hash(block_hash, Full).await
                    {
                        base_fee_per_gas = block.header.base_fee_per_gas;
                    }
                }

                Ok(TransactionDetails::build(transaction, transaction_receipt, base_fee_per_gas))
            }
            else
            {
                Err("Transaction receipt not found".into())
            }
        }
        else
        {
            Err("Transaction not found".into())
        }
    }
    
    fn build(transaction: Transaction, transaction_receipt: TransactionReceipt, base_fee_per_gas: Option<u64>) -> Self
    {
        TransactionDetails //test
        {
            submission_details: SubmissionDetails::build(transaction, base_fee_per_gas),
            outcome_details: OutcomeDetails::from(transaction_receipt),
        }
    }

    pub fn print_transaction_details(&self)
    {
        match self.submission_details.eip_type
        {
            EipType::Legacy => self.print_legacy_transaction_details(),
            EipType::Eip2930 => self.print_legacy_transaction_details(),
            EipType::Eip1559 => self.print_post_1559_transaction_details(),
            EipType::Eip4844 => self.print_post_1559_transaction_details(),
            EipType::Eip4844WithSidecar => self.print_post_1559_transaction_details(),
            EipType::Eip7702 => self.print_post_1559_transaction_details(),
        }
    }

    fn print_legacy_transaction_details(&self)
    {
        let to_field = match self.submission_details.to
        {
            TxKind::Create => "contract created in this transaction",
            TxKind::Call(address) => &format!("{address}"),
        };
        
        println!("\n   Transaction details:\n");
        println!("                     EIP: {:?}", self.submission_details.eip_type);
        println!("       transaction index: {}", self.submission_details.transaction_index.expect("failed to retrieve transaction index"));
        println!("        transaction hash: {}", self.submission_details.transaction_hash);
        println!("                    from: {}", self.submission_details.from);
        println!("                      to: {}", to_field);
        println!("                   value: {}", self.submission_details.value);
        println!("                   input: {}", self.submission_details.input);
        println!("               gas price: {}", self.submission_details.gas_price.expect("failed to retrieve gas price"));
        println!("               gas limit: {}", self.submission_details.gas_limit);
        // println!("         max fee per gas: {}", self.submission_details.max_fee_per_gas);
        // println!("max priority fee per gas: {}", self.submission_details.max_priority_fee_per_gas);
        // println!("        base fee per gas: {}", self.submission_details.base_fee_per_gas);
        // println!("     effective gas price: {}", self.outcome_details.effective_gas_price);
        println!("                gas used: {}", self.outcome_details.gas_used);
        println!("            block number: {}", self.outcome_details.block_number.expect("failed to retrieve block number"));
        println!("              block hash: {}", self.outcome_details.block_hash.expect("failed to retrieve block hash"));        
    }

    fn print_post_1559_transaction_details(&self)
    {
        let to_field = match self.submission_details.to
        {
            TxKind::Create => "contract created in this transaction",
            TxKind::Call(address) => &format!("{address}"),
        };

        println!("\n   Transaction details:\n");
        println!("                     EIP: {:?}", self.submission_details.eip_type);
        println!("       transaction index: {:?}", self.submission_details.transaction_index.expect("failed to retrieve transaction index"));
        println!("        transaction hash: {:?}", self.submission_details.transaction_hash);
        println!("                    from: {:?}", self.submission_details.from);
        println!("                      to: {:?}", to_field);
        println!("                   value: {:?}", self.submission_details.value);
        println!("                   input: {:?}", self.submission_details.input);
        // println!("               gas price: {:?}", self.submission_details.gas_price.expect("failed to retrieve gas price"));
        println!("               gas limit: {:?}", self.submission_details.gas_limit);
        println!("         max fee per gas: {:?}", self.submission_details.max_fee_per_gas);
        println!("max priority fee per gas: {:?}", self.submission_details.max_priority_fee_per_gas.expect("failed to retrieve pax priority fee per gas"));
        println!("        base fee per gas: {:?}", self.submission_details.base_fee_per_gas.expect("failed to retrieve base fee per gas"));
        println!("     effective gas price: {:?}", self.outcome_details.effective_gas_price.expect("failed to retrieve effective gas price"));
        println!("                gas used: {:?}", self.outcome_details.gas_used);
        println!("            block number: {:?}", self.outcome_details.block_number.expect("failed to retrieve block number"));
        println!("              block hash: {:?}", self.outcome_details.block_hash.expect("failed to retrieve block hash"));        
    }

/* 
eip_type,
transaction_index: transaction.transaction_index,
transaction_hash: *transaction.inner.tx_hash(),
from: transaction.from,
to,
value,
input: transaction.input().clone(),
gas_price: transaction.gas_price(),
gas_limit: transaction.gas_limit(),
max_fee_per_gas: transaction.max_fee_per_gas(),
max_priority_fee_per_gas: transaction.max_priority_fee_per_gas(),
base_fee_per_gas,

effective_gas_price: Some(transaction_receipt.effective_gas_price),
gas_used: transaction_receipt.gas_used,
block_number: transaction_receipt.block_number,
block_hash: transaction_receipt.block_hash,

| **Gas Field**                | **Legacy** | **EIP-2930** | **EIP-1559** | **EIP-4844** | **EIP-4844 With Sidecar** | **EIP-7702** |
|------------------------------|------------|--------------|--------------|--------------|---------------------------|--------------|
| **gas_used**                 | ✅ Present | ✅ Present  | ✅ Present   | ✅ Present   | ✅ Present               | ✅ Present   |
| **effective_gas_price**      | ❌ N/A     | ❌ N/A      | ✅ Present   | ✅ Present   | ✅ Present               | ✅ Present   |
| **gas_price**                | ✅ Present | ✅ Present  | ❌ Obsolete  | ❌ Obsolete  | ❌ Obsolete              | ❌ Obsolete  |
| **gas_limit**                | ✅ Present | ✅ Present  | ✅ Present   | ✅ Present   | ✅ Present               | ✅ Present   |
| **max_fee_per_gas**          | ❌ N/A     | ❌ N/A      | ✅ Present   | ✅ Present   | ✅ Present               | ✅ Present   |
| **max_priority_fee_per_gas** | ❌ N/A     | ❌ N/A      | ✅ Present   | ✅ Present   | ✅ Present               | ✅ Present   |
| **base_fee_per_gas**         | ❌ N/A     | ❌ N/A      | ✅ Present   | ✅ Present   | ✅ Present               | ✅ Present   |
 */

    
    pub fn print_details(&self)
    {
        match self.submission_details.eip_type
        {
            EipType::Legacy => todo!(),
            EipType::Eip2930 => todo!(),
            EipType::Eip1559 => todo!(),
            EipType::Eip4844 => todo!(),
            EipType::Eip4844WithSidecar => todo!(),
            EipType::Eip7702 => todo!(),
        }
        println!                                   ("\n Transaction details:\n");
        println!                                   ("                  type: {:?}", self.submission_details.eip_type);
        println!                                   ("      transaction hash: {:?}", self.submission_details.transaction_hash);
        match self.submission_details.transaction_index
        {
            Some(transaction_index)     => println!("     transaction index: {}", transaction_index),
            None                        => println!("     transaction index: no transaction index available"),
        }
        println!                                   ("                  from: {:?}", self.submission_details.from);
        println!                                   ("                    to: {:?}", self.submission_details.to);
        println!                                   ("                 value: {:?}", self.submission_details.value);
        println!                                   ("                 input: {:?}", self.submission_details.input);
        match self.submission_details.gas_price
        {
            Some(gas_price)             => println!("             gas price: {}", gas_price),
            None                        => println!("             gas price: no gas price available"),
        }
        match self.outcome_details.effective_gas_price
        {
            Some(effective_gas_price)   => println!("   effective gas price: {}", effective_gas_price),
            None                        => println!("   effective gas price: no effective gas price available"),
            
        }
        println!                                   ("              gas used: {:?}", self.outcome_details.gas_used);
        match self.outcome_details.block_number
        {
            Some(block_number)          => println!("          block number: {}", block_number),
            None                        => println!("          block number: no block number available"),
        }
        match self.outcome_details.block_hash
        {
            Some(block_hash)            => println!("            block hash: {}", block_hash),
            None                        => println!("            block hash: no block hash available"),
        }
        
        // println!("\nTransaction details:\n");
        // println!("                  type: {:?}", self.submission_details.eip_type);
        // println!("     transaction index: {:?}", self.submission_details.transaction_index);
        // println!("      transaction hash: {:?}", self.submission_details.transaction_hash);
        // println!("                  from: {:?}", self.submission_details.from);
        // println!("                    to: {:?}", self.submission_details.to);
        // println!("                 value: {:?}", self.submission_details.value);
        // println!("             gas price: {:?}", self.submission_details.gas_price);
        // println!("                 input: {:?}", self.submission_details.input);
        // println!("   effective gas price: {:?}", self.outcome_details.effective_gas_price);
        // println!("              gas used: {:?}", self.outcome_details.gas_used);
        // println!("          block number: {:?}", self.outcome_details.block_number);
        // println!("            block hash: {:?}", self.outcome_details.block_hash);
    }
}

#[derive(Debug)]
pub enum EipType
{
    Legacy,
    Eip2930,
    Eip1559,
    Eip4844,
    Eip4844WithSidecar,
    Eip7702,
}

#[derive(Debug)]
pub struct SubmissionDetails
{
    eip_type: EipType,
    transaction_hash: B256,
    transaction_index: Option<u64>,
    from: Address,
    to: TxKind,
    value: U256,
    input: Bytes,
    gas_price: Option<u128>,
    gas_limit: u64,
    max_fee_per_gas: u128,
    max_priority_fee_per_gas: Option<u128>,
    base_fee_per_gas: Option<u64>,
}

/* 
 */

impl SubmissionDetails
{
    pub fn build(transaction: Transaction, base_fee_per_gas: Option<u64>) -> Self
    {
        let (eip_type, to, value) = SubmissionDetails::fill_fields_from_transaction(&transaction);
        
         SubmissionDetails
        {
            eip_type,
            transaction_index: transaction.transaction_index,
            transaction_hash: *transaction.inner.tx_hash(),
            from: transaction.from,
            to,
            value,
            input: transaction.input().clone(),
            gas_price: transaction.gas_price(),
            gas_limit: transaction.gas_limit(),
            max_fee_per_gas: transaction.max_fee_per_gas(),
            max_priority_fee_per_gas: transaction.max_priority_fee_per_gas(),
            base_fee_per_gas,
        }
    }

    pub fn fill_fields_from_transaction(transaction: &Transaction) -> (EipType, TxKind, U256)
    {
        let (eip, to, val) = match &transaction.inner
        {
            TxEnvelope::Legacy(signed_transaction) => 
            (EipType::Legacy,  signed_transaction.tx().to, signed_transaction.tx().value),

            TxEnvelope::Eip2930(signed_transaction) => 
            (EipType::Eip2930,  signed_transaction.tx().to, signed_transaction.tx().value),

            TxEnvelope::Eip1559(signed_transaction) => 
            (EipType::Eip1559,  signed_transaction.tx().to, signed_transaction.tx().value),

            TxEnvelope::Eip4844(signed_transaction_variant) => 
            {
                match signed_transaction_variant.tx()
                {
                    tx_eip4844::TxEip4844Variant::TxEip4844(signed_transaction) => 
                    (EipType::Eip4844,  TxKind::from(signed_transaction.to), signed_transaction.value),

                    tx_eip4844::TxEip4844Variant::TxEip4844WithSidecar(signed_transaction) => 
                    (EipType::Eip4844WithSidecar,  TxKind::from(signed_transaction.tx().to), signed_transaction.tx().value),                    

                }
            },
            TxEnvelope::Eip7702(signed_transaction) => 
            (EipType::Eip7702,  TxKind::from(signed_transaction.tx().to), signed_transaction.tx().value),

        };

        (eip, to, val)
    }  
    
    pub fn find_eip_type_from_transaction(transaction: &Transaction) -> EipType
    {
        match &transaction.inner
        {
            TxEnvelope::Legacy(_) => EipType::Legacy,
            TxEnvelope::Eip2930(_) => EipType::Eip2930,
            TxEnvelope::Eip1559(_) => EipType::Eip1559,
            TxEnvelope::Eip4844(signed_transaction_variant) => 
            {
                match signed_transaction_variant.tx()
                {
                    tx_eip4844::TxEip4844Variant::TxEip4844(_) => EipType::Eip4844,
                    tx_eip4844::TxEip4844Variant::TxEip4844WithSidecar(_) => EipType::Eip4844WithSidecar,
                }
            },
            TxEnvelope::Eip7702(_) => EipType::Eip7702,
        }
    }

    
    pub fn find_recipient_from_transaction(transaction: &Transaction) -> TxKind
    {
        let returned_tx_kind: TxKind = match &transaction.inner
        {
            TxEnvelope::Legacy(signed_transaction) => signed_transaction.tx().to,
            TxEnvelope::Eip2930(signed_transaction) => signed_transaction.tx().to,
            TxEnvelope::Eip1559(signed_transaction) => signed_transaction.tx().to,
            TxEnvelope::Eip4844(signed_transaction_variant) => 
            {
                match signed_transaction_variant.tx()
                {
                    tx_eip4844::TxEip4844Variant::TxEip4844(signed_transaction) => TxKind::from(signed_transaction.to),
                    tx_eip4844::TxEip4844Variant::TxEip4844WithSidecar(signed_transaction) => TxKind::from(signed_transaction.tx().to),
                }
            },
            TxEnvelope::Eip7702(signed_transaction) => TxKind::from(signed_transaction.tx().to),
        };

        returned_tx_kind
    }

    pub fn find_value_from_transaction(transaction: &Transaction) -> U256
    {
        let returned_value: U256 = match &transaction.inner
        {
            TxEnvelope::Legacy(signed_transaction) => signed_transaction.tx().value,
            TxEnvelope::Eip2930(signed_transaction) => signed_transaction.tx().value,
            TxEnvelope::Eip1559(signed_transaction) => signed_transaction.tx().value,
            TxEnvelope::Eip4844(signed_transaction_variant) =>
            {
                match signed_transaction_variant.tx()
                {
                    tx_eip4844::TxEip4844Variant::TxEip4844(signed_transaction) => signed_transaction.value,
                    tx_eip4844::TxEip4844Variant::TxEip4844WithSidecar(signed_transaction) => signed_transaction.tx().value,
                }
            },
            TxEnvelope::Eip7702(signed_transaction) => signed_transaction.tx().value,
        };

        returned_value
    }

    pub fn find_gas_price_from_transaction(transaction: &Transaction) -> Option<u128>
    {
        Some(0)
    }
}

#[derive(Debug)]
pub struct OutcomeDetails
{
    effective_gas_price: Option<u128>,
    gas_used: u64,
    block_number: Option<u64>,
    block_hash: Option<B256>,
}

impl From<TransactionReceipt> for OutcomeDetails
{
    fn from(transaction_receipt: TransactionReceipt) -> Self
    {
        OutcomeDetails
        {
            effective_gas_price: Some(transaction_receipt.effective_gas_price),
            gas_used: transaction_receipt.gas_used,
            block_number: transaction_receipt.block_number,
            block_hash: transaction_receipt.block_hash,
        }
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
    // let parsed_block_number: BlockNumberOrTag = BlockNumberOrTag::Number(block_number);
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

// pub async fn view_block_transactions_data(provider: &RootProvider<Http<Client>>, ident: BlockNumberOrTag) -> Result<BlockTransactionsData, Box<dyn Error>>