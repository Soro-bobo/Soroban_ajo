use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};

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

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
}
