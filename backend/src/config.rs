use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expiry_secs: u64,
    pub jwt_refresh_expiry_secs: u64,
    pub host: String,
    pub port: u16,
    pub frontend_url: String,
    pub horizon_url: String,
    pub soroban_rpc_url: String,
    pub contract_id: String,
    pub app_version: String,
    pub resend_api_key: String,
    pub resend_from_email: String,
    pub email_verification_token_expiry_hours: i64,
    pub password_reset_token_expiry_minutes: i64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: required_env("DATABASE_URL")?,
            jwt_secret: required_env("JWT_SECRET")?,
            jwt_access_expiry_secs: optional_env_parse("JWT_ACCESS_EXPIRY_SECS", 900)?,
            jwt_refresh_expiry_secs: optional_env_parse("JWT_REFRESH_EXPIRY_SECS", 604_800)?,
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: optional_env_parse("PORT", 8080)?,
            frontend_url: std::env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            horizon_url: std::env::var("HORIZON_URL")
                .unwrap_or_else(|_| "https://horizon-testnet.stellar.org".to_string()),
            soroban_rpc_url: std::env::var("SOROBAN_RPC_URL")
                .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".to_string()),
            contract_id: required_env("CONTRACT_ID")?,
            app_version: std::env::var("APP_VERSION").unwrap_or_else(|_| "0.1.0".to_string()),
            resend_api_key: required_env("RESEND_API_KEY")?,
            resend_from_email: std::env::var("RESEND_FROM_EMAIL")
                .unwrap_or_else(|_| "Ajo Platform <onboarding@ajo-platform.com>".to_string()),
            email_verification_token_expiry_hours: optional_env_parse(
                "EMAIL_VERIFICATION_TOKEN_EXPIRY_HOURS",
                24,
            )?,
            password_reset_token_expiry_minutes: optional_env_parse(
                "PASSWORD_RESET_TOKEN_EXPIRY_MINUTES",
                30,
            )?,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn required_env(key: &str) -> Result<String> {
    std::env::var(key).with_context(|| format!("Missing required environment variable: {key}"))
}

fn optional_env_parse<T: std::str::FromStr>(key: &str, default: T) -> Result<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    match std::env::var(key) {
        Ok(val) => val
            .parse::<T>()
            .with_context(|| format!("Failed to parse env var {key}={val}")),
        Err(_) => Ok(default),
    }
}
