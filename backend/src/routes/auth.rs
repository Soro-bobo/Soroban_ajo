use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;

use crate::{
    controllers::auth_controller::{login, refresh, register},
    errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
    models::user::{User, UserPublic},
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

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub wallet_address: Option<String>,
}

pub async fn update_profile_handler(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Json(payload): Json<UpdateProfileRequest>,
) -> AppResult<impl IntoResponse> {
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
}
