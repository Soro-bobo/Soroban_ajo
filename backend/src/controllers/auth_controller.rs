use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use validator::Validate;

use crate::{
    errors::{AppError, AppResult},
    models::user::CreateUserInput,
    services::auth_service::AuthService,
    utils::validators::validate_password_strength,
    AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,
    #[validate(length(min = 2, max = 80, message = "Display name must be 2-80 characters"))]
    pub display_name: String,
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

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    let user = AuthService::register(
        &state.db,
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
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

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
