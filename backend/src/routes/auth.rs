use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    controllers::auth_controller::{
        forgot_password, login, refresh, register, resend_verification, reset_password,
        verify_email,
    },
    errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
    models::user::{User, UserPublic},
    utils::validators::validate_optional_stellar_address,
    AppState,
};

pub async fn me_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
) -> AppResult<impl IntoResponse> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserPublic::from(user)))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    #[validate(custom(function = "validate_optional_stellar_address"))]
    pub wallet_address: Option<String>,
}

pub async fn update_profile_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Json(payload): Json<UpdateProfileRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    if let Some(ref name) = payload.display_name {
        if name.trim().is_empty() || name.len() > 100 {
            return Err(AppError::BadRequest(
                "display_name must be 1-100 characters".to_string(),
            ));
        }
    }

    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET
            display_name  = COALESCE($1, display_name),
            wallet_address = COALESCE($2, wallet_address),
            updated_at    = NOW()
        WHERE id = $3
        RETURNING *
        "#,
    )
    .bind(payload.display_name)
    .bind(payload.wallet_address)
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(UserPublic::from(user)))
}

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/verify-email", post(verify_email))
        .route("/resend-verification", post(resend_verification))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}
