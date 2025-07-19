All the commands that I am building;

- Transfer SOL
solana-cli-tool transfer --to <RECIPIENT_PUBKEY> --amount <SOL_AMOUNT> --keypair <PATH_TO_KEYPAIR>

- Check Balance
solana-cli-tool balance --pubkey <PUBLIC_KEY>

- Generate Keypair
solana-cli-tool keypair --output <PATH>

- Airdrop SOL (Optional)
solana-cli-tool airdrop --pubkey <PUBLIC_KEY> --amount <SOL_AMOUNT>

Using 
solana-sdk
clap
