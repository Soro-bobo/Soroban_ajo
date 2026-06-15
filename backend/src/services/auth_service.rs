use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::user::{CreateUserInput, User, UserPublic},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub struct AuthService;

impl AuthService {
    #[tracing::instrument(skip(pool, input), fields(email = %input.email))]
    pub async fn register(pool: &PgPool, input: CreateUserInput) -> AppResult<UserPublic> {
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE email = $1"
        )
        .bind(&input.email)
        .fetch_one(pool)
        .await?;

        if existing > 0 {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        let password_hash = bcrypt::hash(&input.password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Bcrypt error: {e}")))?;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, display_name, wallet_address, is_active)
            VALUES ($1, $2, $3, $4, $5, true)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&input.email)
        .bind(&password_hash)
        .bind(&input.display_name)
        .bind(&input.wallet_address)
        .fetch_one(pool)
        .await?;

        tracing::info!(user_id = %user.id, "User registered");
        Ok(user.into())
    }

    #[tracing::instrument(skip(pool, password, jwt_secret), fields(email = %email))]
    pub async fn login(
        pool: &PgPool,
        email: &str,
        password: &str,
        jwt_secret: &str,
        access_expiry: u64,
        refresh_expiry: u64,
    ) -> AppResult<TokenPair> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND is_active = true"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

        let valid = bcrypt::verify(password, &user.password_hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Bcrypt verify: {e}")))?;

        if !valid {
            return Err(AppError::Unauthorized("Invalid email or password".to_string()));
        }

        let pair = Self::issue_token_pair(
            &user.id.to_string(),
            &user.email,
            jwt_secret,
            access_expiry,
            refresh_expiry,
        )?;

        tracing::info!(user_id = %user.id, "User logged in");
        Ok(pair)
    }

    #[tracing::instrument(skip(pool, refresh_token, jwt_secret))]
    pub async fn refresh(
        pool: &PgPool,
        refresh_token: &str,
        jwt_secret: &str,
        access_expiry: u64,
        refresh_expiry: u64,
    ) -> AppResult<TokenPair> {
        let claims = Self::verify_token(refresh_token, jwt_secret, "refresh")
            .map_err(|_| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Malformed token".to_string()))?;

        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        let pair = Self::issue_token_pair(
            &user.id.to_string(),
            &user.email,
            jwt_secret,
            access_expiry,
            refresh_expiry,
        )?;

        tracing::info!(user_id = %user.id, "Token refreshed");
        Ok(pair)
    }

    pub fn verify_access_token(token: &str, secret: &str) -> Result<Claims> {
        Self::verify_token(token, secret, "access")
    }

    fn verify_token(token: &str, secret: &str, expected_type: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        if token_data.claims.token_type != expected_type {
            anyhow::bail!("Wrong token type: expected {expected_type}");
        }

        Ok(token_data.claims)
    }

    fn issue_token_pair(
        user_id: &str,
        email: &str,
        secret: &str,
        access_expiry: u64,
        refresh_expiry: u64,
    ) -> AppResult<TokenPair> {
        let now = Utc::now();

        let access_claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::seconds(access_expiry as i64)).timestamp(),
            token_type: "access".to_string(),
        };

        let refresh_claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::seconds(refresh_expiry as i64)).timestamp(),
            token_type: "refresh".to_string(),
        };

        let encoding_key = EncodingKey::from_secret(secret.as_bytes());

        let access_token = encode(&Header::default(), &access_claims, &encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encode: {e}")))?;

        let refresh_token = encode(&Header::default(), &refresh_claims, &encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encode: {e}")))?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: access_expiry as i64,
        })
    }
}
