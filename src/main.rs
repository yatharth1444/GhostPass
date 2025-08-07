mod cli;
mod vault;
mod crypto;
mod clipboard;
mod trust;
mod error;

use clap::Parser;
use secrecy::{SecretString, ExposeSecret};
use anyhow::Result;
use tokio;

use crate::cli::{Cli, Commands};
use crate::vault::Vault;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let master_password = rpassword::prompt_password("Enter master password: ")?;
    let key = derive_key_from_password(&master_password);

    let vault_path = std::path::PathBuf::from("ghostpass_vault.dat");

    let mut vault = Vault::new(key, vault_path)?;

    match &cli.command {
        Commands::Add { key, value } => {
            vault.add(key.to_string(), SecretString::new(value.to_string()))?;
            println!("Added entry: {}", key);
        }
        Commands::Get { key } => {
            let secret = vault.get(key)?;
            println!("Value for {}: {}", key, secret.expose_secret());
            clipboard::shadow_clipboard(secret.expose_secret(), 10).await?;
            println!("Password copied to clipboard for 10 seconds.");
        }
        Commands::Delete { key } => {
            vault.delete(key)?;
            println!("Deleted entry: {}", key);
        }
        Commands::List => {
            let entries = vault.list();
            println!("Vault entries:");
            for key in entries {
                println!("- {}", key);
            }
        }
    }

    Ok(())
}

// Simple key derivation for demo: pad or truncate password ASCII bytes to 32 bytes
fn derive_key_from_password(password: &str) -> Vec<u8> {
    let mut key = vec![0u8; 32];
    let bytes = password.as_bytes();
    for i in 0..bytes.len().min(32) {
        key[i] = bytes[i];
    }
    key
}
