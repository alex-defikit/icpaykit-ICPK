use sha2::{Sha256, Digest};
use hex;

pub fn compute_signature(payload: &[u8], secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    hasher.update(secret.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn verify_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
    let computed = compute_signature(payload, secret);
    computed == signature
}
