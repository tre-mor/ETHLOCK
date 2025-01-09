use alloy::prelude::*;
use alloy_primitives::U256;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    let provider = Http::new("https://site1.moralis-nodes.com/eth/d2efae9b74dc45bf9c161e4b13c2cd86")?;
    
    Ok(())
}