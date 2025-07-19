// handling connection to Devnet 
// and constructing transactions
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use std::path::Path;
use std::fs;
use std::convert::TryFrom;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Signer},
    transaction::Transaction
};
use bs58;
use std::str::FromStr;
use anyhow::{Result};
use solana_system_interface::instruction as system_instruction;

pub fn get_rpc_client() -> RpcClient {
    RpcClient::new("https://api.devnet.solana.com".to_string())
}

pub fn read_base58_keypair(path: &Path) -> Result<Keypair> {
    let encoded = fs::read_to_string(path)?;
    let bytes = bs58::decode(encoded.trim()).into_vec()?;
    let keypair = Keypair::try_from(&bytes[..])?;
    Ok(keypair)
}

pub async fn transfer_sol(to: &str, amount: f64, keypair_path: &std::path::Path) -> Result<()> {
    let client = get_rpc_client();
    let keypair = read_base58_keypair(keypair_path)?;
    let to_pubkey = Pubkey::from_str(to)?;
    let lamports = (amount * 1_000_000_000.0) as u64;
    let recent_blockhash = client.get_latest_blockhash()?;
    // basic SOL transfer instruction using the System Program
    let instruction = system_instruction::transfer(&keypair.pubkey(), &to_pubkey, lamports);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],           // array of 1 here
        Some(&keypair.pubkey()),         // who pays the transaction fee
        &[&keypair],          // signers
        recent_blockhash,
    );
    // Send the transaction to the Devnet and wait for confirmation.
    // On success, it returns a transaction signature (base58 string -> look up on Solana Explorer).
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction successful! Signature: {}", signature);
    Ok(())
}

pub async fn check_balance(pubkey: &str) -> anyhow::Result<()> {
    let client = get_rpc_client();
    let pubkey = Pubkey::from_str(pubkey)?; // Convert string to Pubkey type
    let balance = client.get_balance(&pubkey)?; // 1 SOL = 1_000_000_000 lamports
    println!("Balance: {} SOL", balance as f64 / 1_000_000_000.0);
    Ok(())
}