# SolFort

SolFort is a command-line interface (CLI) wallet for Solana that allows users to generate keypairs, request airdrops, send SOL, and check balances. It's designed to be a simple yet powerful tool for interacting with the Solana blockchain.

## Features

- Generate new Solana keypairs
- Request SOL airdrops (on devnet/testnet)
- Send SOL to other addresses
- Check the balance of any Solana address

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)
- Solana CLI tools (optional, but recommended)

### Building from source

1. Clone the repository:

   ```bash
   git clone https://github.com/codewithmide/solfort
   cd solfort
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/solfort`

## Usage

### General Command Structure

```bash
solfort [COMMAND] [ARGUMENTS]
```

Use `solfort --help` to see a list of all available commands.

### Generate a new keypair

```bash
solfort generate-keypair
```

This will create a new Solana keypair and save it in the `solfort` directory.

### Request an airdrop

```bash
solfort airdrop <AMOUNT> <RECIPIENT_PUBKEY>
```

Example:

```bash
solfort airdrop 1.5 7dE8vUD3vz3jwL6H3kfei2thNwopKGC99H7XE3mfcQu7
```

This requests an airdrop of 1.5 SOL to the specified public key. Note that airdrops are only available on devnet and testnet.

### Send SOL

```bash
solfort send <AMOUNT> <SENDER_PUBKEY> <RECIPIENT_PUBKEY>
```

Example:

```bash
solfort send 0.1 7dE8vUD3vz3jwL6H3kfei2thNwopKGC99H7XE3mfcQu7 AnotherValidPublicKeyHere
```

This sends 0.1 SOL from the sender's address to the recipient's address.

### Check balance

```bash
solfort balance <PUBKEY>
```

If no public key is provided, it will use the default from the config file.

Example:

```bash
solfort balance 7dE8vUD3vz3jwL6H3kfei2thNwopKGC99H7XE3mfcQu7
```

This checks the balance of the specified Solana address.

## Configuration

SolFort uses a configuration file to store default settings. The config file is located at `solfort/config.json` and is created automatically when you first run a command.

You can manually edit this file to change settings such as the default Solana cluster (e.g., devnet, testnet, or mainnet-beta).

## Error Handling

SolFort provides detailed error messages to help you troubleshoot issues. If you encounter an error, read the message carefully for information on how to resolve it.

## Development

SolFort is open source and contributions are welcome! If you're interested in contributing, please fork the repository and submit a pull request.

### Running Tests

To run the test suite:

```bash
cargo test
```

## Contact

For questions, issues, or contributions, please open an issue on the GitHub repository or contact the maintainer at <codewithmide@gmail.com>.

---

Thank you for using SolFort! We hope this tool makes your Solana development experience smoother and more enjoyable.
