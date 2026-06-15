use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    middleware::auth::AuthenticatedUser,
    AppState,
};

pub async fn get_my_membership(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Path(group_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let member = sqlx::query_as::<_, crate::models::member::Member>(
        "SELECT * FROM members WHERE group_id = $1 AND user_id = $2"
    )
    .bind(group_id)
    .bind(auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("You are not a member of this group".to_string()))?;

    Ok(Json(member))
}

pub async fn remove_member(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedUser>,
    Path((group_id, member_id)): Path<(Uuid, Uuid)>,
) -> AppResult<impl IntoResponse> {
    let group = sqlx::query_as::<_, crate::models::group::Group>(
        "SELECT * FROM groups WHERE id = $1"
    )
    .bind(group_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;

    if group.creator_id != auth.user_id {
        return Err(AppError::Forbidden(
            "Only the group creator can remove members".to_string(),
        ));
    }

    let affected = sqlx::query(
        r#"
        UPDATE members
        SET status = 'removed', updated_at = NOW()
        WHERE id = $1 AND group_id = $2 AND user_id != $3
        "#,
    )
    .bind(member_id)
    .bind(group_id)
    .bind(auth.user_id)
    .execute(&state.db)
    .await?
    .rows_affected();

    if affected == 0 {
        return Err(AppError::NotFound("Member not found or cannot be removed".to_string()));
    }

    Ok(Json(json!({ "message": "Member removed" })))
}
