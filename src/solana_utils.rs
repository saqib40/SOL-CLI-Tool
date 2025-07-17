// handling connection to Devnet 
// and constructing transactions
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Signer, read_keypair_file},
    transaction::Transaction,
    system_instruction,
};
use std::str::FromStr;
use anyhow::{anyhow, Result};

pub fn get_rpc_client() -> RpcClient {
    RpcClient::new("https://api.devnet.solana.com".to_string())
}

pub async fn transfer_sol(to: &str, amount: f64, keypair_path: &std::path::Path) -> anyhow::Result<()> {
    let client = get_rpc_client();
    let keypair = read_keypair_file(keypair_path)
        .map_err(|e| anyhow!("Failed to read keypair file: {}", e))?;
    let to_pubkey = Pubkey::from_str(to)?;
    let lamports = (amount * 1_000_000_000.0) as u64;
    // basic SOL transfer instruction using the System Program
    let instruction = system_instruction::transfer(&keypair.pubkey(), &to_pubkey, lamports);
    let recent_blockhash = client.get_latest_blockhash()?;
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