use std::path::Path;
use std::fs;
use crate::solana_utils;
use std::str::FromStr;

pub async fn transfer_sol(to: &str, amount: f64, keypair_path: &Path) -> anyhow::Result<()> {
    solana_utils::transfer_sol(to, amount, keypair_path).await?;
    Ok(())
}

pub async fn check_balance(pubkey: &str) -> anyhow::Result<()> {
    solana_utils::check_balance(pubkey).await
}

pub async fn generate_keypair2(output: &Path) -> anyhow::Result<()> {
    let keypair = solana_sdk::signature::Keypair::new();
    let keypair_bytes = keypair.to_bytes().to_vec();
    let json_string = serde_json::to_string(&keypair_bytes)?;
    fs::write(output, json_string)?;
    println!(
        "Keypair generated and saved to {:?} in the standard JSON format.",
        output
    );
    Ok(())
}

pub async fn request_airdrop(pubkey: &str, amount: f64) -> anyhow::Result<()> {
    let client = solana_utils::get_rpc_client();
    let pubkey = solana_sdk::pubkey::Pubkey::from_str(pubkey)?;
    let lamports = (amount * 1_000_000_000.0) as u64;
    let signature = client.request_airdrop(&pubkey, lamports)?;
    client.confirm_transaction(&signature)?;
    println!("Airdrop successful! Signature: {}", signature);
    Ok(())
}