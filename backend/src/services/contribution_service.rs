use chrono::{Datelike, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::{AppError, AppResult},
    models::contribution::{Contribution, CreateContributionInput, ListContributionsQuery},
    services::stellar_service::StellarService,
    utils::pagination::{clamp_limit, PaginatedResponse},
};

pub struct ContributionService;

impl ContributionService {
    #[tracing::instrument(skip(pool, stellar, input), fields(group_id = %input.group_id, member_id = %member_id))]
    pub async fn record(
        pool: &PgPool,
        stellar: &StellarService,
        member_id: Uuid,
        input: CreateContributionInput,
    ) -> AppResult<Contribution> {
        // Verify member belongs to group
        let member_exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM members WHERE id = $1 AND group_id = $2"
        )
        .bind(member_id)
        .bind(input.group_id)
        .fetch_one(pool)
        .await?;

        if member_exists == 0 {
            return Err(AppError::Forbidden(
                "Not a member of this group".to_string(),
            ));
        }

        // Check for duplicate tx_hash
        let duplicate = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contributions WHERE tx_hash = $1"
        )
        .bind(&input.tx_hash)
        .fetch_one(pool)
        .await?;

        if duplicate > 0 {
            return Err(AppError::Conflict(
                "Transaction already recorded".to_string(),
            ));
        }

        // Verify on Stellar Horizon before persisting
        let verified = stellar.verify_transaction(&input.tx_hash).await?;
        if !verified {
            return Err(AppError::BadRequest(
                "Transaction could not be verified on Stellar".to_string(),
            ));
        }

        // period_date = start of current month in UTC
        let period_date: chrono::DateTime<Utc> = {
            let now = Utc::now();
            chrono::DateTime::<Utc>::from_naive_utc_and_offset(
                chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                chrono::Utc,
            )
        };

        let contribution = sqlx::query_as::<_, Contribution>(
            r#"
            INSERT INTO contributions (
                id, group_id, member_id, amount, tx_hash, status, period_date, confirmed_at
            )
            VALUES ($1, $2, $3, $4, $5, 'confirmed', $6, $7)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(input.group_id)
        .bind(member_id)
        .bind(input.amount)
        .bind(&input.tx_hash)
        .bind(period_date)
        .bind(Utc::now())
        .fetch_one(pool)
        .await?;

        tracing::info!(
            contribution_id = %contribution.id,
            tx_hash = %input.tx_hash,
            "Contribution recorded"
        );
        Ok(contribution)
    }

    #[tracing::instrument(skip(pool))]
    pub async fn list_for_group(
        pool: &PgPool,
        group_id: Uuid,
        query: ListContributionsQuery,
    ) -> AppResult<PaginatedResponse<Contribution>> {
        let limit = clamp_limit(query.limit);
        let fetch_limit = limit + 1;

        let contributions = match (query.cursor, query.member_id, query.status) {
            (Some(cursor), Some(mid), Some(status)) => {
                sqlx::query_as::<_, Contribution>(
                    r#"
                    SELECT * FROM contributions
                    WHERE group_id = $1 AND id > $2 AND member_id = $3 AND status = $4
                    ORDER BY id ASC LIMIT $5
                    "#,
                )
                .bind(group_id)
                .bind(cursor)
                .bind(mid)
                .bind(status)
                .bind(fetch_limit)
                .fetch_all(pool)
                .await?
            }
            (Some(cursor), None, None) => {
                sqlx::query_as::<_, Contribution>(
                    r#"
                    SELECT * FROM contributions
                    WHERE group_id = $1 AND id > $2
                    ORDER BY id ASC LIMIT $3
                    "#,
                )
                .bind(group_id)
                .bind(cursor)
                .bind(fetch_limit)
                .fetch_all(pool)
                .await?
            }
            _ => {
                sqlx::query_as::<_, Contribution>(
                    "SELECT * FROM contributions WHERE group_id = $1 ORDER BY id ASC LIMIT $2"
                )
                .bind(group_id)
                .bind(fetch_limit)
                .fetch_all(pool)
                .await?
            }
        };

        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contributions WHERE group_id = $1"
        )
        .bind(group_id)
        .fetch_one(pool)
        .await?;

        let last_id = contributions.last().map(|c| c.id);
        Ok(PaginatedResponse::new(contributions, limit, total, last_id))
    }

    #[tracing::instrument(skip(pool))]
    pub async fn detect_missed(pool: &PgPool) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            UPDATE contributions
            SET status = 'missed', updated_at = NOW()
            WHERE status = 'pending'
              AND period_date < NOW() - INTERVAL '48 hours'
            "#,
        )
        .execute(pool)
        .await?;

        let count = result.rows_affected();
        if count > 0 {
            tracing::info!(count = %count, "Marked missed contributions");
        }
        Ok(count)
    }
}
