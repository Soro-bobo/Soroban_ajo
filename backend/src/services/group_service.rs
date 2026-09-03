use chrono::{Datelike, NaiveDate, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::{
        group::{ContributionFrequency, CreateGroupInput, Group, GroupStatus, ListGroupsQuery},
        member::{Member, MemberWithUser},
    },
    utils::pagination::{clamp_limit, PaginatedResponse},
};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PayoutScheduleRow {
    pub id: Uuid,
    pub group_id: Uuid,
    pub member_id: Uuid,
    pub display_name: String,
    pub payout_round: i32,
    pub scheduled_date: NaiveDate,
    pub paid_at: Option<chrono::DateTime<Utc>>,
    pub reminder_sent_at: Option<chrono::DateTime<Utc>>,
    pub amount: Option<rust_decimal::Decimal>,
}

pub struct GroupService;

impl GroupService {
    #[tracing::instrument(skip(pool, input), fields(creator_id = %creator_id, name = %input.name))]
    pub async fn create(
        pool: &PgPool,
        creator_id: Uuid,
        input: CreateGroupInput,
    ) -> AppResult<Group> {
        if input.max_members < 2 || input.max_members > 50 {
            return Err(AppError::BadRequest(
                "max_members must be between 2 and 50".to_string(),
            ));
        }
        if input.contribution_amount <= rust_decimal::Decimal::ZERO {
            return Err(AppError::BadRequest(
                "contribution_amount must be greater than 0".to_string(),
            ));
        }

        // Insert group + creator member in a transaction; return the fully updated group
        let mut tx = pool.begin().await?;

        let group_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO groups (
                id, name, description, contribution_amount, frequency,
                max_members, current_members, status, start_date, creator_id,
                current_payout_position
            )
            VALUES ($1, $2, $3, $4, $5, $6, 1, 'PENDING', $7, $8, 0)
            "#,
        )
        .bind(group_id)
        .bind(&input.name)
        .bind(&input.description)
        .bind(input.contribution_amount)
        .bind(&input.frequency)
        .bind(input.max_members)
        .bind(input.start_date)
        .bind(creator_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO members (id, group_id, user_id, payout_position, status) VALUES ($1, $2, $3, 1, 'active')",
        )
        .bind(member_id)
        .bind(group_id)
        .bind(creator_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        let group = Self::get_by_id(pool, group_id).await?;
        tracing::info!(group_id = %group_id, "Group created");
        Ok(group)
    }

    #[tracing::instrument(skip(pool), fields(group_id = %group_id, user_id = %user_id))]
    pub async fn join(pool: &PgPool, group_id: Uuid, user_id: Uuid) -> AppResult<Member> {
        let group = Self::get_by_id(pool, group_id).await?;

        if group.status != GroupStatus::Pending && group.status != GroupStatus::Active {
            return Err(AppError::BadRequest(
                "Group is not accepting new members".to_string(),
            ));
        }
        if group.current_members >= group.max_members {
            return Err(AppError::Conflict("Group is full".to_string()));
        }

        let already = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM members WHERE group_id = $1 AND user_id = $2",
        )
        .bind(group_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        if already > 0 {
            return Err(AppError::Conflict(
                "Already a member of this group".to_string(),
            ));
        }

        let next_position = group.current_members + 1;
        let mut tx = pool.begin().await?;

        let member = sqlx::query_as::<_, Member>(
            "INSERT INTO members (id, group_id, user_id, payout_position, status) VALUES ($1, $2, $3, $4, 'active') RETURNING *",
        )
        .bind(Uuid::new_v4()).bind(group_id).bind(user_id).bind(next_position)
        .fetch_one(&mut *tx).await?;

        sqlx::query("UPDATE groups SET current_members = current_members + 1, updated_at = $1 WHERE id = $2")
            .bind(Utc::now()).bind(group_id)
            .execute(&mut *tx).await?;

        tx.commit().await?;
        tracing::info!(group_id = %group_id, user_id = %user_id, position = %next_position, "Member joined");
        Ok(member)
    }

    #[tracing::instrument(skip(pool), fields(group_id = %group_id, creator_id = %creator_id))]
    pub async fn activate(pool: &PgPool, group_id: Uuid, creator_id: Uuid) -> AppResult<Group> {
        let group = Self::get_by_id(pool, group_id).await?;
        if group.creator_id != creator_id {
            return Err(AppError::Forbidden(
                "Only the creator can activate this group".to_string(),
            ));
        }
        if group.status != GroupStatus::Pending {
            return Err(AppError::BadRequest(
                "Group must be in PENDING status to activate".to_string(),
            ));
        }
        if group.current_members < 2 {
            return Err(AppError::BadRequest(
                "Group needs at least 2 members to activate".to_string(),
            ));
        }

        let mut tx = pool.begin().await?;

        sqlx::query("UPDATE groups SET status = 'ACTIVE', current_payout_position = 1, updated_at = $1 WHERE id = $2")
            .bind(Utc::now()).bind(group_id)
            .execute(&mut *tx).await?;

        // Generate payout schedule for all active members
        let members = sqlx::query_as::<_, crate::models::member::Member>(
            "SELECT * FROM members WHERE group_id = $1 AND status = 'active' ORDER BY payout_position ASC",
        )
        .bind(group_id).fetch_all(&mut *tx).await?;

        for member in &members {
            let scheduled_date =
                compute_payout_date(group.start_date, member.payout_position, &group.frequency);
            let payout_amount =
                group.contribution_amount * rust_decimal::Decimal::from(members.len() as i64);

            sqlx::query(
                r#"
                INSERT INTO payout_schedule (id, group_id, member_id, payout_round, scheduled_date, amount)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (group_id, payout_round) DO NOTHING
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(group_id)
            .bind(member.id)
            .bind(member.payout_position)
            .bind(scheduled_date)
            .bind(payout_amount)
            .execute(&mut *tx).await?;
        }

        tx.commit().await?;
        tracing::info!(group_id = %group_id, "Group activated");
        Self::get_by_id(pool, group_id).await
    }

    #[tracing::instrument(skip(pool), fields(group_id = %id))]
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> AppResult<Group> {
        sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Group {id} not found")))
    }

    #[tracing::instrument(skip(pool))]
    pub async fn list(
        pool: &PgPool,
        query: ListGroupsQuery,
    ) -> AppResult<PaginatedResponse<Group>> {
        let limit = clamp_limit(query.limit);
        let fetch_limit = limit + 1;

        let groups = match (query.cursor, query.status) {
            (Some(cursor), Some(status)) => {
                sqlx::query_as::<_, Group>(
                    "SELECT * FROM groups WHERE id > $1 AND status = $2 ORDER BY created_at DESC LIMIT $3",
                )
                .bind(cursor).bind(status).bind(fetch_limit).fetch_all(pool).await?
            }
            (Some(cursor), None) => {
                sqlx::query_as::<_, Group>(
                    "SELECT * FROM groups WHERE id > $1 ORDER BY created_at DESC LIMIT $2",
                )
                .bind(cursor).bind(fetch_limit).fetch_all(pool).await?
            }
            (None, Some(status)) => {
                sqlx::query_as::<_, Group>(
                    "SELECT * FROM groups WHERE status = $1 ORDER BY created_at DESC LIMIT $2",
                )
                .bind(status).bind(fetch_limit).fetch_all(pool).await?
            }
            (None, None) => {
                sqlx::query_as::<_, Group>(
                    "SELECT * FROM groups ORDER BY created_at DESC LIMIT $1",
                )
                .bind(fetch_limit).fetch_all(pool).await?
            }
        };

        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM groups")
            .fetch_one(pool)
            .await?;
        let last_id = groups.last().map(|g| g.id);
        Ok(PaginatedResponse::new(groups, limit, total, last_id))
    }

    #[tracing::instrument(skip(pool), fields(group_id = %group_id))]
    pub async fn list_members(pool: &PgPool, group_id: Uuid) -> AppResult<Vec<MemberWithUser>> {
        let _ = Self::get_by_id(pool, group_id).await?;
        let members = sqlx::query_as::<_, MemberWithUser>(
            r#"
            SELECT m.id, m.group_id, m.user_id,
                   u.display_name, u.email, u.wallet_address,
                   m.payout_position, m.status, m.has_received_payout, m.joined_at
            FROM members m
            JOIN users u ON u.id = m.user_id
            WHERE m.group_id = $1
            ORDER BY m.payout_position ASC
            "#,
        )
        .bind(group_id)
        .fetch_all(pool)
        .await?;
        Ok(members)
    }

    #[tracing::instrument(skip(pool), fields(group_id = %group_id))]
    pub async fn get_payout_schedule(
        pool: &PgPool,
        group_id: Uuid,
    ) -> AppResult<Vec<PayoutScheduleRow>> {
        let _ = Self::get_by_id(pool, group_id).await?;
        let rows = sqlx::query_as::<_, PayoutScheduleRow>(
            r#"
            SELECT ps.id, ps.group_id, ps.member_id,
                   u.display_name,
                   ps.payout_round, ps.scheduled_date, ps.paid_at, ps.reminder_sent_at, ps.amount
            FROM payout_schedule ps
            JOIN members m ON m.id = ps.member_id
            JOIN users u ON u.id = m.user_id
            WHERE ps.group_id = $1
            ORDER BY ps.payout_round ASC
            "#,
        )
        .bind(group_id)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    #[tracing::instrument(skip(pool), fields(group_id = %group_id))]
    pub async fn advance_payout(pool: &PgPool, group_id: Uuid) -> AppResult<()> {
        let group = Self::get_by_id(pool, group_id).await?;
        if group.status != GroupStatus::Active {
            return Err(AppError::BadRequest("Group is not active".to_string()));
        }

        let next_position = group.current_payout_position + 1;
        let new_status = if next_position > group.current_members {
            GroupStatus::Completed
        } else {
            GroupStatus::Active
        };

        let mut tx = pool.begin().await?;

        sqlx::query(
            "UPDATE groups SET current_payout_position = $1, status = $2, updated_at = $3 WHERE id = $4",
        )
        .bind(next_position).bind(new_status).bind(Utc::now()).bind(group_id)
        .execute(&mut *tx).await?;

        sqlx::query(
            "UPDATE members SET has_received_payout = true, updated_at = $1 WHERE group_id = $2 AND payout_position = $3",
        )
        .bind(Utc::now()).bind(group_id).bind(group.current_payout_position)
        .execute(&mut *tx).await?;

        sqlx::query(
            "UPDATE payout_schedule SET paid_at = $1 WHERE group_id = $2 AND payout_round = $3",
        )
        .bind(Utc::now())
        .bind(group_id)
        .bind(group.current_payout_position)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        tracing::info!(group_id = %group_id, next_position = %next_position, "Payout advanced");
        Ok(())
    }
}

fn compute_payout_date(
    start: NaiveDate,
    round: i32,
    frequency: &ContributionFrequency,
) -> NaiveDate {
    match frequency {
        ContributionFrequency::Weekly => start + chrono::Duration::weeks((round - 1) as i64),
        ContributionFrequency::Biweekly => {
            start + chrono::Duration::weeks(((round - 1) * 2) as i64)
        }
        ContributionFrequency::Monthly => {
            let months = (round - 1) as u32;
            let y = start.year() + ((start.month0() + months) / 12) as i32;
            let m = (start.month0() + months) % 12 + 1;
            NaiveDate::from_ymd_opt(y, m, start.day()).unwrap_or(start)
        }
    }
}
