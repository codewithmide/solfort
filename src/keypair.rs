use anyhow::{anyhow, Result};
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use std::fs;
use std::path::Path;

pub fn generate_keypair() -> Result<()> {
    let keypair: Keypair = Keypair::new();
    let solfort_dir: &Path = Path::new("solfort");

    fs::create_dir_all(solfort_dir)?;

    let public_key: String = keypair.pubkey().to_string();
    let secret_key: String = keypair.to_base58_string();

    fs::write(solfort_dir.join("publicKey.txt"), public_key)?;
    fs::write(solfort_dir.join("secretKey.txt"), secret_key)?;

    println!("Keypair generated and saved in the 'solfort' directory.");
    Ok(())
}

pub fn read_keypair(pubkey: &str) -> Result<Keypair> {
    let solfort_dir: &Path = Path::new("solfort");
    let secret_key_path: std::path::PathBuf = solfort_dir.join("secretKey.txt");

    if !secret_key_path.exists() {
        return Err(anyhow!(
            "Secret key file not found. Generate a keypair first."
        ));
    }

    let secret_key: String = fs::read_to_string(secret_key_path)?;
    let keypair: Keypair = Keypair::from_base58_string(&secret_key);

    if keypair.pubkey().to_string() != pubkey {
        return Err(anyhow!(
            "Provided public key does not match the stored keypair."
        ));
    }

    Ok(keypair)
}
