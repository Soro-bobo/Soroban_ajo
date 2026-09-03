use std::borrow::Cow;
use validator::ValidationError;

use crate::utils::password_blocklist::is_common_password;

/// Validates a Stellar public key (G...) including its checksum, via `stellar-strkey`
/// rather than a length/regex check, since a regex can't catch a corrupted checksum.
pub fn validate_stellar_address(address: &str) -> Result<(), ValidationError> {
    stellar_strkey::ed25519::PublicKey::from_string(address).map_err(|_| {
        let mut err = ValidationError::new("invalid_stellar_address");
        err.message = Some(Cow::from("Invalid Stellar wallet address"));
        err
    })?;
    Ok(())
}

/// Only validates when a wallet address is actually provided — the field is optional
/// at registration and users can connect via Freighter later. `validator`'s derive
/// macro already skips `None`, so this only ever runs against `Some(value)`.
pub fn validate_optional_stellar_address(address: &str) -> Result<(), ValidationError> {
    if address.is_empty() {
        return Ok(());
    }
    validate_stellar_address(address)
}

pub fn validate_tx_hash(hash: &str) -> Result<(), ValidationError> {
    if hash.len() != 64 {
        return Err(ValidationError::new("invalid_tx_hash_length"));
    }
    let valid_hex = hash.chars().all(|c| c.is_ascii_hexdigit());
    if !valid_hex {
        return Err(ValidationError::new("invalid_tx_hash_hex"));
    }
    Ok(())
}

pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_upper {
        let mut err = ValidationError::new("password_missing_uppercase");
        err.message = Some(Cow::from(
            "Password must include at least one uppercase letter",
        ));
        return Err(err);
    }
    if !has_lower {
        let mut err = ValidationError::new("password_missing_lowercase");
        err.message = Some(Cow::from(
            "Password must include at least one lowercase letter",
        ));
        return Err(err);
    }
    if !has_digit {
        let mut err = ValidationError::new("password_missing_digit");
        err.message = Some(Cow::from("Password must include at least one number"));
        return Err(err);
    }
    Ok(())
}

pub fn validate_password_not_common(password: &str) -> Result<(), ValidationError> {
    if is_common_password(password) {
        let mut err = ValidationError::new("password_too_common");
        err.message = Some(Cow::from(
            "This password is too common and appears in known data breaches — choose another",
        ));
        return Err(err);
    }
    Ok(())
}
