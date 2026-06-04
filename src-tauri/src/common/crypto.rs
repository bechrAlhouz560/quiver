use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::Argon2;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

// Derives a 32-byte encryption key from a master password using Argon2id.
// Salt should be stored alongside the encrypted data (not secret).
pub(crate) fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], String> {
    let mut key = [0u8; 32];

    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| e.to_string())?;

    Ok(key)
}

// Generates a random 16-byte salt. Call once per vault, store it.
pub(crate) fn generate_salt() -> [u8; 16] {
    use aes_gcm::aead::rand_core::RngCore;
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}

// Encrypts a plaintext string with AES-256-GCM.
// Returns base64(nonce + ciphertext) everything needed to decrypt later.
pub(crate) fn encrypt(key: &[u8; 32], plaintext: &str) -> Result<String, String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    // random 12-byte nonce — unique per encryption
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| e.to_string())?;

    // prepend nonce to ciphertext so we can extract it on decrypt
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(combined))
}

// Decrypts a base64(nonce + ciphertext) string produced by `encrypt`.
pub(crate) fn decrypt(key: &[u8; 32], encoded: &str) -> Result<String, String> {
    let combined = BASE64.decode(encoded).map_err(|e| e.to_string())?;

    // first 12 bytes = nonce, rest = ciphertext
    if combined.len() < 12 {
        return Err("Invalid ciphertext: too short".into());
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed: wrong password or corrupted data".to_string())?;

    String::from_utf8(plaintext).map_err(|e| e.to_string())
}
