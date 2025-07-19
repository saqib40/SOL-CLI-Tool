use std::path::Path;
use std::fs;
use crate::solana_utils;
use std::str::FromStr;
use bs58;

pub async fn transfer_sol(to: &str, amount: f64, keypair_path: &Path) -> anyhow::Result<()> {
    solana_utils::transfer_sol(to, amount, keypair_path).await;
    Ok(())
}

pub async fn check_balance(pubkey: &str) -> anyhow::Result<()> {
    solana_utils::check_balance(pubkey).await
}

pub async fn generate_keypair(output: &Path) -> anyhow::Result<()> {
    // Generate a new random keypair (public + private key)
    // This is your Solana wallet, created in-memory
    let keypair = solana_sdk::signature::Keypair::new();
    // Convert keypair to byte array, then encode to base58 string
    let serialized = bs58::encode(keypair.to_bytes()).into_string();

    fs::write(output, serialized)?;

    println!("Keypair generated and saved to {:?}", output);
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