use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{
    errors::{AppError, AppResult},
    middleware::rate_limit::SlidingWindowLimiter,
    models::user::CreateUserInput,
    services::auth_service::AuthService,
    utils::validators::{
        validate_optional_stellar_address, validate_password_not_common, validate_password_strength,
    },
    AppState,
};

const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(15 * 60);
const RATE_LIMIT_MAX_ATTEMPTS: usize = 5;

fn enforce_auth_rate_limit(
    limiter: &SlidingWindowLimiter,
    scope: &str,
    ip: &str,
    email: &str,
) -> AppResult<()> {
    let ip_ok = limiter.check(
        &format!("{scope}:ip:{ip}"),
        RATE_LIMIT_MAX_ATTEMPTS,
        RATE_LIMIT_WINDOW,
    );
    let email_ok = limiter.check(
        &format!("{scope}:email:{email}"),
        RATE_LIMIT_MAX_ATTEMPTS,
        RATE_LIMIT_WINDOW,
    );

    if !ip_ok || !email_ok {
        tracing::warn!(scope, ip, email, "Auth rate limit exceeded");
        return Err(AppError::TooManyRequests);
    }

    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom(function = "validate_password_strength"))]
    #[validate(custom(function = "validate_password_not_common"))]
    pub password: String,
    #[validate(length(min = 2, max = 80, message = "Display name must be 2-80 characters"))]
    pub display_name: String,
    #[validate(custom(function = "validate_optional_stellar_address"))]
    pub wallet_address: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmailRequest {
    #[validate(length(min = 1))]
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResendVerificationRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1))]
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom(function = "validate_password_strength"))]
    #[validate(custom(function = "validate_password_not_common"))]
    pub new_password: String,
}

pub async fn register(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    let email = AuthService::normalize_email(&payload.email);
    enforce_auth_rate_limit(
        &state.auth_rate_limiter,
        "register",
        &addr.ip().to_string(),
        &email,
    )?;

    let user = AuthService::register(
        &state.db,
        &state.config,
        CreateUserInput {
            email: payload.email,
            password: payload.password,
            display_name: payload.display_name,
            wallet_address: payload.wallet_address,
        },
    )
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    let email = AuthService::normalize_email(&payload.email);
    enforce_auth_rate_limit(
        &state.auth_rate_limiter,
        "login",
        &addr.ip().to_string(),
        &email,
    )?;

    let token_pair = AuthService::login(
        &state.db,
        &payload.email,
        &payload.password,
        &state.config.jwt_secret,
        state.config.jwt_access_expiry_secs,
        state.config.jwt_refresh_expiry_secs,
    )
    .await?;

    Ok(Json(token_pair))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    let token_pair = AuthService::refresh(
        &state.db,
        &payload.refresh_token,
        &state.config.jwt_secret,
        state.config.jwt_access_expiry_secs,
        state.config.jwt_refresh_expiry_secs,
    )
    .await?;

    Ok(Json(token_pair))
}

pub async fn verify_email(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    AuthService::verify_email(&state.db, &payload.token).await?;

    Ok(Json(json!({ "message": "Email verified" })))
}

pub async fn resend_verification(
    State(state): State<AppState>,
    Json(payload): Json<ResendVerificationRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    let email = AuthService::normalize_email(&payload.email);
    let allowed = state.auth_rate_limiter.check(
        &format!("resend_verification:email:{email}"),
        1,
        Duration::from_secs(60),
    );
    if !allowed {
        return Err(AppError::TooManyRequests);
    }

    AuthService::resend_verification(&state.db, &state.config, &payload.email).await?;

    Ok(Json(json!({
        "message": "If that email needs verification, a new link has been sent"
    })))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    let email = AuthService::normalize_email(&payload.email);
    enforce_auth_rate_limit(
        &state.auth_rate_limiter,
        "forgot_password",
        &addr.ip().to_string(),
        &email,
    )?;

    AuthService::forgot_password(&state.db, &state.config, &payload.email).await?;

    Ok(Json(json!({
        "message": "If that email is registered, a reset link has been sent"
    })))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    AuthService::reset_password(&state.db, &payload.token, &payload.new_password).await?;

    Ok(Json(json!({ "message": "Password reset successfully" })))
}
