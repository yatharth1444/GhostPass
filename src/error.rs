use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum GhostPassError {
    #[error("Encryption/Decryption error")]
    CryptoError,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Entry not found: {0}")]
    EntryNotFound(String),
    #[error("Clipboard error")]
    ClipboardError,
    #[error("Other error: {0}")]
    Other(String),
}
