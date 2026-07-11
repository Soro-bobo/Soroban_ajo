use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::errors::{AppError, AppResult};

pub struct PasswordService;

impl PasswordService {
    /// Argon2id with a fresh random salt per password (embedded in the PHC-format hash).
    pub fn hash(password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Argon2 hash error: {e}")))
    }

    /// Constant-time verification via argon2's own comparison — never compare hashes with `==`.
    pub fn verify(password: &str, hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Malformed password hash: {e}")))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
