use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    errors::{AppError, AppResult},
    models::user::{CreateUserInput, User, UserPublic},
    services::{email_service::EmailService, password_service::PasswordService},
    utils::tokens::{generate_opaque_token, hash_token},
};

const GENERIC_LOGIN_ERROR: &str = "Invalid email or password";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub struct AuthService;

impl AuthService {
    pub fn normalize_email(email: &str) -> String {
        email.trim().to_lowercase()
    }

    #[tracing::instrument(skip(pool, config, input), fields(email = %input.email))]
    pub async fn register(
        pool: &PgPool,
        config: &Config,
        input: CreateUserInput,
    ) -> AppResult<UserPublic> {
        let email = Self::normalize_email(&input.email);

        let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind(&email)
            .fetch_one(pool)
            .await?;

        if existing > 0 {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        let password_hash = PasswordService::hash(&input.password)?;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, display_name, wallet_address, is_active, email_verified)
            VALUES ($1, $2, $3, $4, $5, true, false)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&email)
        .bind(&password_hash)
        .bind(&input.display_name)
        .bind(&input.wallet_address)
        .fetch_one(pool)
        .await?;

        tracing::info!(user_id = %user.id, "User registered");

        if let Err(e) = Self::issue_and_send_verification_email(pool, config, &user).await {
            // Registration already succeeded — the user can retry via /auth/resend-verification.
            tracing::error!(error = %e, user_id = %user.id, "Failed to send verification email");
        }

        Ok(user.into())
    }

    async fn issue_and_send_verification_email(
        pool: &PgPool,
        config: &Config,
        user: &User,
    ) -> AppResult<()> {
        let token = generate_opaque_token();
        let expires_at = Utc::now() + Duration::hours(config.email_verification_token_expiry_hours);

        sqlx::query(
            "INSERT INTO email_verification_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(Uuid::new_v4())
        .bind(user.id)
        .bind(&token.hash)
        .bind(expires_at)
        .execute(pool)
        .await?;

        EmailService::send_verification_email(
            &config.resend_api_key,
            &config.resend_from_email,
            &config.frontend_url,
            &user.email,
            &user.display_name,
            &token.plain,
        )
        .await
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
        let email = Self::normalize_email(email);

        let user =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 AND is_active = true")
                .bind(&email)
                .fetch_optional(pool)
                .await?
                .ok_or_else(|| AppError::Unauthorized(GENERIC_LOGIN_ERROR.to_string()))?;

        // Same generic error whether the email doesn't exist or the password is wrong,
        // so login failures can't be used to enumerate registered accounts.
        let valid = PasswordService::verify(password, &user.password_hash)?;
        if !valid {
            return Err(AppError::Unauthorized(GENERIC_LOGIN_ERROR.to_string()));
        }

        let pair = Self::issue_token_pair(
            pool,
            &user.id,
            &user.email,
            Uuid::new_v4(),
            jwt_secret,
            access_expiry,
            refresh_expiry,
        )
        .await?;

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
        let presented_hash = hash_token(refresh_token);

        let row = sqlx::query_as::<_, RefreshTokenRow>(
            "SELECT id, user_id, family_id, revoked_at, expires_at FROM refresh_tokens WHERE token_hash = $1",
        )
        .bind(&presented_hash)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

        if row.revoked_at.is_some() {
            // This token was already rotated out — someone is replaying an old refresh
            // token, which is a compromise signal. Kill the whole family.
            tracing::warn!(user_id = %row.user_id, family_id = %row.family_id, "Refresh token reuse detected — revoking session family");
            sqlx::query(
                "UPDATE refresh_tokens SET revoked_at = NOW() WHERE family_id = $1 AND revoked_at IS NULL",
            )
            .bind(row.family_id)
            .execute(pool)
            .await?;

            return Err(AppError::Unauthorized(
                "Session invalidated — please log in again".to_string(),
            ));
        }

        if row.expires_at < Utc::now() {
            return Err(AppError::Unauthorized(
                "Invalid or expired refresh token".to_string(),
            ));
        }

        let user =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 AND is_active = true")
                .bind(row.user_id)
                .fetch_optional(pool)
                .await?
                .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE id = $1")
            .bind(row.id)
            .execute(pool)
            .await?;

        let pair = Self::issue_token_pair(
            pool,
            &user.id,
            &user.email,
            row.family_id,
            jwt_secret,
            access_expiry,
            refresh_expiry,
        )
        .await?;

        tracing::info!(user_id = %user.id, "Token refreshed");
        Ok(pair)
    }

    #[tracing::instrument(skip(pool, token))]
    pub async fn verify_email(pool: &PgPool, token: &str) -> AppResult<()> {
        let token_hash = hash_token(token);

        let row = sqlx::query_as::<_, VerificationTokenRow>(
            "SELECT id, user_id, expires_at FROM email_verification_tokens WHERE token_hash = $1 AND used_at IS NULL",
        )
        .bind(&token_hash)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired verification token".to_string()))?;

        if row.expires_at < Utc::now() {
            return Err(AppError::BadRequest(
                "Invalid or expired verification token".to_string(),
            ));
        }

        let mut tx = pool.begin().await?;

        sqlx::query("UPDATE users SET email_verified = true, updated_at = NOW() WHERE id = $1")
            .bind(row.user_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "UPDATE email_verification_tokens SET used_at = NOW() WHERE user_id = $1 AND used_at IS NULL",
        )
        .bind(row.user_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        tracing::info!(user_id = %row.user_id, "Email verified");
        Ok(())
    }

    #[tracing::instrument(skip(pool, config), fields(email = %email))]
    pub async fn resend_verification(pool: &PgPool, config: &Config, email: &str) -> AppResult<()> {
        let email = Self::normalize_email(email);

        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND is_active = true AND email_verified = false",
        )
        .bind(&email)
        .fetch_optional(pool)
        .await?;

        // Silently no-op if the email doesn't exist or is already verified — the
        // response is identical either way to avoid leaking account existence.
        if let Some(user) = user {
            if let Err(e) = Self::issue_and_send_verification_email(pool, config, &user).await {
                tracing::error!(error = %e, user_id = %user.id, "Failed to resend verification email");
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(pool, config), fields(email = %email))]
    pub async fn forgot_password(pool: &PgPool, config: &Config, email: &str) -> AppResult<()> {
        let email = Self::normalize_email(email);

        let user =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 AND is_active = true")
                .bind(&email)
                .fetch_optional(pool)
                .await?;

        if let Some(user) = user {
            let token = generate_opaque_token();
            let expires_at =
                Utc::now() + Duration::minutes(config.password_reset_token_expiry_minutes);

            let result: AppResult<()> = async {
                sqlx::query(
                    "INSERT INTO password_reset_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, $4)",
                )
                .bind(Uuid::new_v4())
                .bind(user.id)
                .bind(&token.hash)
                .bind(expires_at)
                .execute(pool)
                .await?;

                EmailService::send_password_reset_email(
                    &config.resend_api_key,
                    &config.resend_from_email,
                    &config.frontend_url,
                    &user.email,
                    &user.display_name,
                    &token.plain,
                )
                .await
            }
            .await;

            if let Err(e) = result {
                tracing::error!(error = %e, user_id = %user.id, "Failed to send password reset email");
            }
        }

        // Same response whether or not the email exists.
        Ok(())
    }

    #[tracing::instrument(skip(pool, token, new_password))]
    pub async fn reset_password(pool: &PgPool, token: &str, new_password: &str) -> AppResult<()> {
        let token_hash = hash_token(token);

        let row = sqlx::query_as::<_, VerificationTokenRow>(
            "SELECT id, user_id, expires_at FROM password_reset_tokens WHERE token_hash = $1 AND used_at IS NULL",
        )
        .bind(&token_hash)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid or expired reset token".to_string()))?;

        if row.expires_at < Utc::now() {
            return Err(AppError::BadRequest(
                "Invalid or expired reset token".to_string(),
            ));
        }

        let password_hash = PasswordService::hash(new_password)?;

        let mut tx = pool.begin().await?;

        sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
            .bind(&password_hash)
            .bind(row.user_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "UPDATE password_reset_tokens SET used_at = NOW() WHERE user_id = $1 AND used_at IS NULL",
        )
        .bind(row.user_id)
        .execute(&mut *tx)
        .await?;

        // A password reset invalidates every existing session — if an attacker had a
        // stolen refresh token, this cuts it off.
        sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL")
            .bind(row.user_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        tracing::info!(user_id = %row.user_id, "Password reset, all sessions invalidated");
        Ok(())
    }

    /// Gates actions (creating/joining a group) that require a confirmed email.
    /// Checked against the DB rather than the JWT claim so verification takes
    /// effect immediately instead of waiting for the access token to expire.
    pub async fn require_verified(pool: &PgPool, user_id: Uuid) -> AppResult<()> {
        let verified =
            sqlx::query_scalar::<_, bool>("SELECT email_verified FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(pool)
                .await?
                .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if !verified {
            return Err(AppError::Forbidden(
                "Please verify your email before creating or joining groups".to_string(),
            ));
        }

        Ok(())
    }

    pub fn verify_access_token(token: &str, secret: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }

    async fn issue_token_pair(
        pool: &PgPool,
        user_id: &Uuid,
        email: &str,
        family_id: Uuid,
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
        };

        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let access_token = encode(&Header::default(), &access_claims, &encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT encode: {e}")))?;

        let refresh = generate_opaque_token();
        let refresh_expires_at = now + Duration::seconds(refresh_expiry as i64);

        sqlx::query(
            "INSERT INTO refresh_tokens (id, user_id, family_id, token_hash, expires_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(family_id)
        .bind(&refresh.hash)
        .bind(refresh_expires_at)
        .execute(pool)
        .await?;

        Ok(TokenPair {
            access_token,
            refresh_token: refresh.plain,
            expires_in: access_expiry as i64,
        })
    }
}

#[derive(sqlx::FromRow)]
struct RefreshTokenRow {
    id: Uuid,
    user_id: Uuid,
    family_id: Uuid,
    revoked_at: Option<chrono::DateTime<Utc>>,
    expires_at: chrono::DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct VerificationTokenRow {
    #[allow(dead_code)]
    id: Uuid,
    user_id: Uuid,
    expires_at: chrono::DateTime<Utc>,
}
