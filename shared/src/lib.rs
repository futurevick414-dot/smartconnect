pub mod protocol;

use sha2::{Digest, Sha256};

pub fn sign_payload(payload: &str, secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(secret);
    hasher.update(payload);
    hex::encode(hasher.finalize())
}

pub fn verify_payload(payload: &str, signature: &str, secret: &str) -> bool {
    sign_payload(payload, secret) == signature
}