use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};

/// A freshly generated opaque token: `plain` is returned to the caller (email link,
/// response body) and never persisted; `hash` is what gets stored in the DB.
pub struct GeneratedToken {
    pub plain: String,
    pub hash: String,
}

pub fn generate_opaque_token() -> GeneratedToken {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let plain = URL_SAFE_NO_PAD.encode(bytes);
    let hash = hash_token(&plain);
    GeneratedToken { plain, hash }
}

pub fn hash_token(plain: &str) -> String {
    let digest = Sha256::digest(plain.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}
