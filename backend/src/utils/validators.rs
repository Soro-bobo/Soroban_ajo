use validator::ValidationError;

pub fn validate_stellar_address(address: &str) -> Result<(), ValidationError> {
    if address.len() != 56 {
        return Err(ValidationError::new("invalid_stellar_address_length"));
    }
    if !address.starts_with('G') {
        return Err(ValidationError::new("invalid_stellar_address_prefix"));
    }
    let valid_chars = address.chars().all(|c| {
        matches!(c, 'A'..='Z' | '2'..='7')
    });
    if !valid_chars {
        return Err(ValidationError::new("invalid_stellar_address_chars"));
    }
    Ok(())
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
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    if !has_upper || !has_lower || !has_digit {
        return Err(ValidationError::new("password_too_weak"));
    }
    Ok(())
}
