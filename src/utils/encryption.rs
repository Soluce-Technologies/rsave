use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as Engine;
use rand::RngCore;

/* ---------- crypto helpers ---------- */
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let argon2 = argon2::Argon2::default();
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .unwrap();
    key
}

pub fn encrypt(key: &[u8; 32], plaintext: &str) -> String {
    let cipher = Aes256Gcm::new(&aes_gcm::Key::<Aes256Gcm>::from_slice(key));
    let mut nonce = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_bytes())
        .unwrap();
    format!("{}:{}", Engine.encode(&nonce), Engine.encode(&ciphertext))
}

pub fn decrypt(key: &[u8; 32], s: &str) -> Result<String, ()> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(());
    }
    let nonce = Engine.decode(parts[0]).map_err(|_| ())?;
    let ct = Engine.decode(parts[1]).map_err(|_| ())?;
    let cipher = Aes256Gcm::new(&aes_gcm::Key::<Aes256Gcm>::from_slice(key));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ct.as_ref())
        .map_err(|_| ())?;
    String::from_utf8(plaintext).map_err(|_| ())
}
