use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
mod config;
mod keypair;
mod transactions;

#[derive(Parser)]
#[command(
    name = "solfort",
    author = "codewithmide codewithmide@gmail.com",
    version = "0.1.0",
    about = "A CLI wallet for Solana that can generate keypairs, request airdrops, send SOL, and check balances.",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new keypair
    GenerateKeypair,

    /// Request an airdrop of SOL
    Airdrop {
        /// Amount of SOL to request
        amount: f64,
        /// Recipient's public key
        recipient: String,
    },

    /// Send SOL to another address
    Send {
        /// Amount of SOL to send
        amount: f64,
        /// Sender's public key
        sender: String,
        /// Recipient's public key
        recipient: String,
    },

    /// Check the balance of an address
    Balance {
        /// Public key to check (optional, uses default if not provided)
        pubkey: Option<String>,
    },
}

fn main() {
    let cli = Cli::try_parse();

    match cli {
        Ok(cli) => {
            let result: Result<()> = match &cli.command {
                Some(Commands::GenerateKeypair) => keypair::generate_keypair(),
                Some(Commands::Airdrop { amount, recipient }) => {
                    transactions::request_airdrop(*amount, recipient)
                }
                Some(Commands::Send {
                    amount,
                    sender,
                    recipient,
                }) => transactions::send_sol(*amount, sender, recipient),
                Some(Commands::Balance { pubkey }) => transactions::get_balance(pubkey.as_deref()),
                None => {
                    println!("{}", generate_help_text());
                    Ok(())
                }
            };

            if let Err(e) = result {
                eprintln!("Error: {}", e);
                if e.to_string().contains("Invalid public key") {
                    eprintln!("\nCorrect usage for balance: solfort balance <PUBKEY>");
                    eprintln!(
                        "Example: solfort balance 7dE8vUD3vz3jwL6H3kfei2thNwopKGC99H7XE3mfcQu7\n"
                    );
                }
                if let Some(cause) = e.source() {
                    eprintln!("Caused by: {}", cause);
                }
                std::process::exit(1);
            }
        }
        Err(e) => {
            if e.kind() == clap::error::ErrorKind::ValueValidation {
                eprintln!("Error: Invalid input - {}", e);
                if e.to_string().contains("<AMOUNT>") {
                    eprintln!(
                        "\nCorrect usage for airdrop: solfort airdrop <AMOUNT> <RECIPIENT_PUBKEY>"
                    );
                    eprintln!(
                        "Example: solfort airdrop 1.5 7dE8vUD3vz3jwL6H3kfei2thNwopKGC99H7XE3mfcQu7"
                    );
                } else {
                    eprintln!("\nFor more information, try 'solfort --help'");
                }
            } else {
                eprintln!("{}", e);
                eprintln!("\nFor more information, try 'solfort --help'");
            }
            std::process::exit(1);
        }
    }
}

fn generate_help_text() -> String {
    let mut app: clap::Command = Cli::command();
    let mut help_text: Vec<u8> = Vec::new();
    app.write_long_help(&mut help_text).unwrap();
    String::from_utf8(help_text).unwrap()
}
