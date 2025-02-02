use std::error::Error;

use alloy::
{
    consensus::
    {
        transaction::eip4844, Transaction as ConsensusTransaction, TxEnvelope
    }, 
    primitives::
    {
        Address, Bytes, TxKind, B256, U256
    }, 
    providers::Provider,
    rpc::types::
    {
        BlockTransactionsKind, Transaction, TransactionReceipt
    }
};

use crate::provider_info::GenericProvider;

/// A struct that contains the details of a transaction.
#[derive(Debug)]
pub struct TransactionDetails
{
    submission_details: SubmissionDetails,
    outcome_details: OutcomeDetails,
}

impl TransactionDetails
{
    /// Retrieves the details of a transaction from the blockchain.
    pub async fn get(provider: &GenericProvider, transaction_hash: B256) -> Result<TransactionDetails, Box<dyn Error>>
    {
        if let Ok(Some(transaction)) = provider.get_transaction_by_hash(transaction_hash).await
        {
            if let Ok(Some(transaction_receipt)) = provider.get_transaction_receipt(transaction_hash).await
            {
                let mut base_fee_per_gas = None;

                if let Some(block_hash) = transaction_receipt.block_hash
                {
                    if let Ok(Some(block)) = provider.get_block_by_hash(block_hash, BlockTransactionsKind::Full).await
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
    
    /// Builds a new `TransactionDetails` instance from a transaction and a transaction receipt.
    pub fn build(transaction: Transaction, transaction_receipt: TransactionReceipt, base_fee_per_gas: Option<u64>) -> Self
    {
        TransactionDetails
        {
            submission_details: SubmissionDetails::build(transaction, base_fee_per_gas),
            outcome_details: OutcomeDetails::from(transaction_receipt),
        }
    }

    /// Prints the details of the transaction.
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

    /// Prints the details of a legacy transaction.
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

    /// Prints the details of a post-1559 transaction.
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

impl SubmissionDetails
{
    /// Builds a new `SubmissionDetails` instance from a transaction.
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

    /// Fills the fields of a `SubmissionDetails` instance from a transaction.
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
                    eip4844::TxEip4844Variant::TxEip4844(signed_transaction) => 
                    (EipType::Eip4844,  TxKind::from(signed_transaction.to), signed_transaction.value),

                    eip4844::TxEip4844Variant::TxEip4844WithSidecar(signed_transaction) => 
                    (EipType::Eip4844WithSidecar,  TxKind::from(signed_transaction.tx().to), signed_transaction.tx().value),                    

                }
            },
            TxEnvelope::Eip7702(signed_transaction) => 
            (EipType::Eip7702,  TxKind::from(signed_transaction.tx().to), signed_transaction.tx().value),

        };

        (eip, to, val)
    }  
    
    /// Finds the EIP type of a transaction.
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
                    eip4844::TxEip4844Variant::TxEip4844(_) => EipType::Eip4844,
                    eip4844::TxEip4844Variant::TxEip4844WithSidecar(_) => EipType::Eip4844WithSidecar,
                }
            },
            TxEnvelope::Eip7702(_) => EipType::Eip7702,
        }
    }

    
    /// Finds the recipient of a transaction.
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
                    eip4844::TxEip4844Variant::TxEip4844(signed_transaction) => TxKind::from(signed_transaction.to),
                    eip4844::TxEip4844Variant::TxEip4844WithSidecar(signed_transaction) => TxKind::from(signed_transaction.tx().to),
                }
            },
            TxEnvelope::Eip7702(signed_transaction) => TxKind::from(signed_transaction.tx().to),
        };

        returned_tx_kind
    }

    /// Finds the value of a transaction.
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
                    eip4844::TxEip4844Variant::TxEip4844(signed_transaction) => signed_transaction.value,
                    eip4844::TxEip4844Variant::TxEip4844WithSidecar(signed_transaction) => signed_transaction.tx().value,
                }
            },
            TxEnvelope::Eip7702(signed_transaction) => signed_transaction.tx().value,
        };

        returned_value
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
