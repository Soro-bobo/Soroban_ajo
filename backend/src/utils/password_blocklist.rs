use once_cell::sync::Lazy;
use std::collections::HashSet;

const RAW_LIST: &str = include_str!("../data/common_passwords.txt");

static BLOCKLIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    RAW_LIST
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect()
});

/// Case-insensitive membership check against the top 10k breached-password list
/// (SecLists `10k-most-common.txt`), so obvious-but-technically-valid passwords
/// like "Password1" are rejected even though they pass the complexity rules.
pub fn is_common_password(password: &str) -> bool {
    BLOCKLIST.contains(password.to_lowercase().as_str())
}
