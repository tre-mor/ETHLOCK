mod key_info;
mod block_info;
mod provider_info;
mod transaction_info;
mod rpc_client;

use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // let provider = provider_info::build_provider_from_url_str("https://site1.moralis-nodes.com/eth/d2efae9b74dc45bf9c161e4b13c2cd86")?;

    // let tx = 
    //     transaction_info::TransactionDetails::get
    //     (
    //         &provider, alloy::primitives::B256::from_str("0x291351476ef62e83ed33fb385f998232b8577bd1af60eb3463ce5a9e77fc8666"
    //     ).unwrap()).await;

    // if let Ok(tx) = tx
    // {
    //     tx.print_transaction_details();
    // }

    let generic_signer = key_info::generate_new_signer();

    println!("private key: {}", key_info::derive_private_key(&generic_signer));

    println!("private key: {}", key_info::derive_private_key_as_string(&generic_signer));

    println!("private key: {:#}", key_info::derive_private_key_as_bytes(&generic_signer));

    println!("private key: {}", key_info::derive_private_key_as_bytes(&generic_signer));
    
    println!("private key: {}", &key_info::derive_private_key_as_bytes(&generic_signer).to_string());

    println!("private key: {}", &key_info::derive_private_key_as_bytes(&generic_signer).to_string()[2..]);

    println!("private key: {}", key_info::remove_hex_prefix_from_fixed_bytes(&key_info::derive_private_key_as_bytes(&generic_signer)));

    println!("    address: {}", key_info::derive_address_from_generic_signer(&generic_signer));

    println!("    address: {:#}", key_info::derive_address_from_generic_signer(&generic_signer));

    println!("    address: {:?}", key_info::derive_address_from_generic_signer(&generic_signer));

    println!("    address: {:#?}", key_info::derive_address_from_generic_signer(&generic_signer));

    println!("    address: {}", key_info::remove_hex_prefix_from_fixed_bytes
        (
            &key_info::derive_address_from_generic_signer(&generic_signer)
        )
    );

    println!(" public key: {}", key_info::derive_verifying_key_from_signing_key(&generic_signer));

        // let rpc_url: reqwest::Url =
    //     "https://site1.moralis-nodes.com/eth/d2efae9b74dc45bf9c161e4b13c2cd86".parse()?;
    // let provider = ProviderBuilder::new().on_http(rpc_url);
// let block_option = provider.get_block_by_number(Latest, Full).await?;

    // if let Some(block) = block_option
    // {
    //     let btxdet: block_info::BlockTransactionsDetails =
    // block_info::BlockTransactionsDetails::build_struct(&provider, block).await?;

    //     for transaction in btxdet.transactions
    //     {
    //         transaction.print_details();
    //     }
    // }

    // if let Ok(latest_block_number) = block_info::get_latest_block_number(&provider).await
    // {
    //     println!("latest block number: {}", latest_block_number);

    //     // if let Ok(latest_block) = block_info::build_block_struct(&provider,
    // latest_block_number).await     // {
    //     //     println!("latest block: {:#?}", latest_block);
    //     // }

    //     if let Ok(latest_block) = block_info::build_block_struct_simple(&provider,
    // Number(latest_block_number)).await     {
    //         println!("latest block: {:#?}", latest_block);
    //     }
    // }

    // if let Ok(latest_block_number) = block_info::get_latest_block_number(&provider).await
    // {
    //     println!("latest block number: {}", latest_block_number);

    //     if let Ok(latest_n_block_numbers) =
    // block_info::get_latest_n_block_numbers(latest_block_number, 5).await     {
    //         println!("previous {} blocks: {:#?}", latest_n_block_numbers.len(),
    // latest_n_block_numbers);     }
    // }

    // println!("{:?}", view_block_header_data(&provider, Latest).await);

    Ok(())
}

// I am building an Ethereum block explorer with Rust, alloy.rs (a rust crate for interfacing with
// the Ethereum blockchain), Tauri, and Swift. The block explorer will display block number, hash,
// timestamp, and transaction details. It will also implement basic filtering. Remember to refer to
// the documentation (the entire repo in the alloy directory and the alloy_rs_book) when writing the
// code. Prioritize efficiency and clarity. All code must be well explained with comments on every
// line.
