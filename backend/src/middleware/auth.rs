use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use uuid::Uuid;

use crate::{errors::AppError, services::auth_service::AuthService, AppState};

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub email: String,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = AuthService::verify_access_token(bearer.token(), &state.config.jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Malformed token subject".to_string()))?;

    req.extensions_mut().insert(AuthenticatedUser {
        user_id,
        email: claims.email,
    });

    Ok(next.run(req).await)
}
