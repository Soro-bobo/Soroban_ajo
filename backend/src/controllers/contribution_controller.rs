use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
    models::contribution::{CreateContributionInput, ListContributionsQuery},
    services::contribution_service::ContributionService,
    utils::validators::validate_tx_hash,
    AppState,
};

#[derive(Debug, serde::Deserialize, Validate)]
pub struct RecordContributionRequest {
    pub group_id: Uuid,
    #[validate(custom(function = "validate_tx_hash"))]
    pub tx_hash: String,
    pub amount: rust_decimal::Decimal,
}

pub async fn record_contribution(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Json(payload): Json<RecordContributionRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    if payload.amount <= rust_decimal::Decimal::ZERO {
        return Err(AppError::BadRequest("amount must be positive".to_string()));
    }

    let member = sqlx::query_as::<_, crate::models::member::Member>(
        "SELECT * FROM members WHERE group_id = $1 AND user_id = $2",
    )
    .bind(payload.group_id)
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Forbidden("Not a member of this group".to_string()))?;

    let contribution = ContributionService::record(
        &state.db,
        &state.stellar,
        member.id,
        CreateContributionInput {
            group_id: payload.group_id,
            amount: payload.amount,
            tx_hash: payload.tx_hash,
        },
    )
    .await?;

    Ok((StatusCode::CREATED, Json(contribution)))
}

pub async fn list_contributions(
    State(state): State<AppState>,
    Path(group_id): Path<Uuid>,
    Query(query): Query<ListContributionsQuery>,
) -> AppResult<impl IntoResponse> {
    let page = ContributionService::list_for_group(&state.db, group_id, query).await?;
    Ok(Json(page))
}
