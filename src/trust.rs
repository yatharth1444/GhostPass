use sha2::{Digest, Sha256};
use hex;
#[allow(dead_code)]
pub fn generate_fingerprint(pub_key: &[u8]) -> String {
    let hash = Sha256::digest(pub_key);
    hex::encode(&hash[..6])
}
