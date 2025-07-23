use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod commands;
mod solana_utils;

#[derive(Parser, Debug)]
#[command(name = "solana-cli-tool", about = "A CLI tool for Solana transactions")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// ... to write
    Transfer {
        #[arg(short, long, help = "Recipient public key")]
        to: String,
        #[arg(short, long, help = "Amount of SOL to transfer")]
        amount: f64,
        #[arg(short, long, help = "Path to sender's keypair JSON")]
        keypair: PathBuf,
    },
    /// ...
    Balance {
        #[arg(short, long, help = "Public key to check balance")]
        pubkey: String,
    },
    /// ...
    Keypair {
        #[arg(short, long, help = "Output path for keypair JSON")]
        output: PathBuf,
    },
    /// ...
    Airdrop {
        #[arg(short, long, help = "Public key to receive airdrop")]
        pubkey: String,
        #[arg(short, long, help = "Amount of SOL to request")]
        amount: f64,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Transfer { to, amount, keypair } => {
            commands::transfer_sol(&to, amount, &keypair).await?;
        }
        Commands::Balance { pubkey } => {
            commands::check_balance(&pubkey).await?;
        }
        Commands::Keypair { output } => {
            commands::generate_keypair2(&output).await?;
        }
        Commands::Airdrop { pubkey, amount } => {
            commands::request_airdrop(&pubkey, amount).await?;
        }
    }
    Ok(())
}