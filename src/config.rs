use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub cluster: String,
    pub pubkey: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cluster: "https://api.devnet.solana.com".to_string(),
            pubkey: "".to_string(),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config_path: std::path::PathBuf = Path::new("solfort").join("config.json");
    if config_path.exists() {
        let config_str = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&config_str)?)
    } else {
        let config = Config::default();
        save_config(&config)?;
        Ok(config)
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path: std::path::PathBuf = Path::new("solfort").join("config.json");
    let config_str: String = serde_json::to_string_pretty(config)?;
    fs::write(config_path, config_str)?;
    Ok(())
}
