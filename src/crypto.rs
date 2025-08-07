use secrecy::{ExposeSecret, SecretString};
use ring::aead::*;
use ring::rand::{SystemRandom, SecureRandom};
use anyhow::{Result, anyhow};
use arrayref::array_ref;

const TAG_LEN: usize = 16;
const NONCE_LEN: usize = 12;

pub fn encrypt(key: &[u8], plaintext: &SecretString) -> Result<Vec<u8>> {
    let sealing_key = LessSafeKey::new(
        UnboundKey::new(&AES_256_GCM, key).map_err(|_| anyhow!("Invalid key"))?
    );
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes).map_err(|_| anyhow!("Nonce generation failed"))?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let mut in_out = plaintext.expose_secret().as_bytes().to_vec();
    in_out.extend_from_slice(&[0u8; TAG_LEN]);

    sealing_key
        .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| anyhow!("Encryption failed"))?;

    let mut result = nonce_bytes.to_vec();
    result.extend(in_out);
    Ok(result)
}

pub fn decrypt(key: &[u8], ciphertext: &[u8]) -> Result<SecretString> {
    if ciphertext.len() < NONCE_LEN + TAG_LEN {
        return Err(anyhow!("Ciphertext too short"));
    }
    let nonce_bytes = &ciphertext[..NONCE_LEN];
    let nonce = Nonce::assume_unique_for_key(*array_ref!(nonce_bytes, 0, NONCE_LEN));
    let mut in_out = ciphertext[NONCE_LEN..].to_vec();

    let opening_key = LessSafeKey::new(
        UnboundKey::new(&AES_256_GCM, key).map_err(|_| anyhow!("Invalid key"))?
    );

    let decrypted_bytes = opening_key
        .open_in_place(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| anyhow!("Decryption failed"))?;

    let decrypted_str = std::str::from_utf8(decrypted_bytes)?;
    Ok(SecretString::new(decrypted_str.to_string()))
}
