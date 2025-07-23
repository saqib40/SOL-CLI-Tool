# solana-cli-tool

## Made With :

- **clap** 

- **solana-sdk**

## Run Locally

1 - Clone the project
```bash
  git clone https://github.com/your-username/ble-evidence-platform.git
  cd ble-evidence-platform
```

2 - Build the project
```bash
  cargo build
```

3 - Run the project
- Transfer SOL
```bash
cargo run -- transfer --to <RECIPIENT_PUBKEY> --amount <SOL_AMOUNT> --keypair <PATH_TO_KEYPAIR>
```

- Check Balance
```bash
cargo run -- balance --pubkey <PUBLIC_KEY>
```

- Generate Keypair
```bash
cargo run -- keypair --output <PATH> (path can be any filename.json)
```

- Airdrop SOL (Optional)
```bash
cargo run -- airdrop --pubkey <PUBLIC_KEY> --amount <SOL_AMOUNT>
```
- To create pubkey from keypair
```bash
solana-keygen pubkey my_keypair.json
```