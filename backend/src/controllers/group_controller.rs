use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
    models::group::{CreateGroupInput, ListGroupsQuery},
    services::group_service::GroupService,
    AppState,
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGroupRequest {
    #[validate(length(min = 2, max = 100, message = "Name must be 2-100 characters"))]
    pub name: String,
    #[validate(length(max = 500, message = "Description must be under 500 characters"))]
    pub description: Option<String>,
    pub contribution_amount: rust_decimal::Decimal,
    pub frequency: crate::models::group::ContributionFrequency,
    #[validate(range(min = 2, max = 50, message = "max_members must be 2-50"))]
    pub max_members: i32,
    pub start_date: chrono::NaiveDate,
}

pub async fn create_group(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Json(payload): Json<CreateGroupRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(format!("{e}")))?;

    if payload.contribution_amount <= rust_decimal::Decimal::ZERO {
        return Err(AppError::BadRequest(
            "contribution_amount must be greater than 0".to_string(),
        ));
    }

    let group = GroupService::create(
        &state.db,
        auth.user_id,
        CreateGroupInput {
            name: payload.name,
            description: payload.description,
            contribution_amount: payload.contribution_amount,
            frequency: payload.frequency,
            max_members: payload.max_members,
            start_date: payload.start_date,
        },
    )
    .await?;

    Ok((StatusCode::CREATED, Json(group)))
}

pub async fn get_group(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let group = GroupService::get_by_id(&state.db, id).await?;
    Ok(Json(group))
}

pub async fn list_groups(
    State(state): State<AppState>,
    Query(query): Query<ListGroupsQuery>,
) -> AppResult<impl IntoResponse> {
    let page = GroupService::list(&state.db, query).await?;
    Ok(Json(page))
}

pub async fn join_group(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let member = GroupService::join(&state.db, id, auth.user_id).await?;
    Ok((StatusCode::CREATED, Json(member)))
}

pub async fn list_group_members(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let members = GroupService::list_members(&state.db, id).await?;
    Ok(Json(members))
}

pub async fn activate_group(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let group = GroupService::activate(&state.db, id, auth.user_id).await?;
    Ok(Json(group))
}

pub async fn get_payout_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let schedule = GroupService::get_payout_schedule(&state.db, id).await?;
    Ok(Json(schedule))
}

pub async fn get_group_stats(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let group = GroupService::get_by_id(&state.db, id).await?;

    let total_contributed = sqlx::query_scalar::<_, Option<rust_decimal::Decimal>>(
        "SELECT SUM(amount) FROM contributions WHERE group_id = $1 AND status = 'confirmed'"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?
    .unwrap_or(rust_decimal::Decimal::ZERO);

    let contribution_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM contributions WHERE group_id = $1 AND status = 'confirmed'"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    let expected_total = group.contribution_amount
        * rust_decimal::Decimal::from(group.current_members)
        * rust_decimal::Decimal::from(group.current_payout_position.max(1));

    Ok(Json(serde_json::json!({
        "group_id": id,
        "total_contributed": total_contributed,
        "contribution_count": contribution_count,
        "expected_total": expected_total,
        "completion_rate": if expected_total > rust_decimal::Decimal::ZERO {
            (total_contributed / expected_total * rust_decimal::Decimal::from(100)).round()
        } else {
            rust_decimal::Decimal::ZERO
        },
        "current_round": group.current_payout_position,
        "total_rounds": group.current_members,
    })))
}

pub async fn advance_payout(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let group = GroupService::get_by_id(&state.db, id).await?;
    if group.creator_id != auth.user_id {
        return Err(crate::errors::AppError::Forbidden(
            "Only the group creator can advance the payout".to_string(),
        ));
    }
    GroupService::advance_payout(&state.db, id).await?;
    let updated = GroupService::get_by_id(&state.db, id).await?;
    Ok(Json(updated))
}
