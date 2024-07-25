use crate::config;
use crate::keypair;
use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature;
use solana_sdk::signer::Signer;
use solana_sdk::{message::Message, pubkey::Pubkey, system_instruction, transaction::Transaction};
use std::str::FromStr;

pub fn request_airdrop(amount: f64, recipient: &str) -> Result<()> {
    let config: config::Config = config::load_config()?;
    let client: RpcClient = RpcClient::new(&config.cluster);

    let recipient_pubkey: Pubkey = Pubkey::from_str(recipient)
        .map_err(|_| anyhow!("Invalid recipient public key: {}", recipient))?;

    let amount_lamports: u64 = (amount * 1_000_000_000.0) as u64;

    let signature: signature::Signature = client
        .request_airdrop(&recipient_pubkey, amount_lamports)
        .map_err(|e: solana_client::client_error::ClientError| {
            anyhow!("Airdrop request failed: {}", e)
        })?;

    client.confirm_transaction(&signature).map_err(
        |e: solana_client::client_error::ClientError| {
            anyhow!("Failed to confirm transaction: {}", e)
        },
    )?;

    println!("Airdrop of {} SOL to {} successful", amount, recipient);
    Ok(())
}

pub fn send_sol(amount: f64, sender: &str, recipient: &str) -> Result<()> {
    let config: config::Config = config::load_config()?;
    let client: RpcClient = RpcClient::new(&config.cluster);
    let sender_keypair: solana_sdk::signature::Keypair = keypair::read_keypair(sender)?;
    let recipient_pubkey: Pubkey = recipient.parse::<Pubkey>()?;

    let amount_lamports: u64 = (amount * 1_000_000_000.0) as u64;
    let instruction: solana_sdk::instruction::Instruction =
        system_instruction::transfer(&sender_keypair.pubkey(), &recipient_pubkey, amount_lamports);

    let message: Message = Message::new(&[instruction], Some(&sender_keypair.pubkey()));
    let transaction: Transaction =
        Transaction::new(&[&sender_keypair], message, client.get_latest_blockhash()?);

    let signature: signature::Signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction successful. Signature: {}", signature);

    Ok(())
}

pub fn get_balance(pubkey: Option<&str>) -> Result<()> {
    let config: config::Config = config::load_config()?;
    let client: RpcClient = RpcClient::new(&config.cluster);

    let pubkey: Pubkey = match pubkey {
        Some(key_str) => {
            Pubkey::from_str(key_str).map_err(|e: solana_sdk::pubkey::ParsePubkeyError| {
                anyhow!("Invalid public key: {}. Error: {}", key_str, e)
            })?
        }
        None => config
            .pubkey
            .parse()
            .map_err(|e: solana_sdk::pubkey::ParsePubkeyError| {
                anyhow!(
                    "Invalid public key in config: {}. Error: {}",
                    config.pubkey,
                    e
                )
            })?,
    };

    match client.get_balance(&pubkey) {
        Ok(balance) => {
            println!(
                "Balance of {}: {} SOL",
                pubkey,
                balance as f64 / 1_000_000_000.0
            );
            Ok(())
        }
        Err(e) => Err(anyhow!("Failed to get balance: {}", e)),
    }
}
