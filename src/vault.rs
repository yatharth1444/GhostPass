use crate::crypto::{encrypt, decrypt};
use secrecy::{SecretString, ExposeSecret};
use anyhow::{Result, Context};
use std::{collections::HashMap, fs, path::PathBuf};

pub struct Vault {
    store: HashMap<String, SecretString>,
    key: Vec<u8>, // symmetric key
    path: PathBuf,
}

impl Vault {
    pub fn new(key: Vec<u8>, path: PathBuf) -> Result<Self> {
        let mut vault = Vault {
            store: HashMap::new(),
            key,
            path,
        };
        vault.load().ok(); // Ignore load errors on first run
        Ok(vault)
    }

    pub fn load(&mut self) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }
        let encrypted_data = fs::read(&self.path).context("Failed to read vault file")?;
        let decrypted = decrypt(&self.key, &encrypted_data).context("Failed to decrypt vault file")?;
        let serialized = decrypted.expose_secret();

        // Deserialize to HashMap<String, String> then convert to SecretString
        let tmp_store: HashMap<String, String> = serde_json::from_str(serialized)
            .context("Failed to parse vault JSON")?;
        self.store = tmp_store.into_iter()
            .map(|(k, v)| (k, SecretString::new(v)))
            .collect();
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        // Convert HashMap<String, SecretString> -> HashMap<String, String>
        let tmp_store: HashMap<String, String> = self.store.iter()
            .map(|(k, v)| (k.clone(), v.expose_secret().clone()))
            .collect();

        let serialized = serde_json::to_string(&tmp_store).context("Failed to serialize vault")?;
        let secret_serialized = SecretString::new(serialized);
        let encrypted = encrypt(&self.key, &secret_serialized).context("Failed to encrypt vault")?;
        fs::write(&self.path, encrypted).context("Failed to write vault file")?;
        Ok(())
    }

    pub fn add(&mut self, key: String, value: SecretString) -> Result<()> {
        self.store.insert(key, value);
        self.save()
    }

    pub fn get(&self, key: &str) -> Result<&SecretString> {
        self.store.get(key).ok_or_else(|| anyhow::anyhow!("Entry not found: {}", key))
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        if self.store.remove(key).is_none() {
            return Err(anyhow::anyhow!("Entry not found: {}", key));
        }
        self.save()?;
        Ok(())
    }

    pub fn list(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.store.keys().cloned().collect();
        keys.sort();
        keys
    }
}
