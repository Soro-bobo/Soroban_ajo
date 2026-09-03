use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::Config, errors::AppResult, services::email_service::EmailService};

#[derive(Debug, sqlx::FromRow)]
struct DueReminderRow {
    id: Uuid,
    display_name: String,
    email: String,
    group_name: String,
    scheduled_date: chrono::NaiveDate,
    amount: Option<rust_decimal::Decimal>,
}

#[derive(Debug, sqlx::FromRow)]
struct PayoutRecipientRow {
    display_name: String,
    email: String,
    group_name: String,
    amount: Option<rust_decimal::Decimal>,
}

fn format_amount(amount: Option<rust_decimal::Decimal>) -> String {
    amount
        .map(|a| a.normalize().to_string())
        .unwrap_or_else(|| "—".to_string())
}

pub struct NotificationService;

impl NotificationService {
    /// Emails members whose payout is scheduled within the next 2 days and
    /// haven't been reminded yet. Best-effort per row: a failed send is logged
    /// and skipped rather than aborting the whole batch.
    #[tracing::instrument(skip(pool, config))]
    pub async fn send_due_reminders(pool: &PgPool, config: &Config) -> AppResult<u64> {
        let rows = sqlx::query_as::<_, DueReminderRow>(
            r#"
            SELECT ps.id, u.display_name, u.email, g.name AS group_name,
                   ps.scheduled_date, ps.amount
            FROM payout_schedule ps
            JOIN members m ON m.id = ps.member_id
            JOIN users u ON u.id = m.user_id
            JOIN groups g ON g.id = ps.group_id
            WHERE ps.paid_at IS NULL
              AND ps.reminder_sent_at IS NULL
              AND ps.scheduled_date <= (NOW() + INTERVAL '2 days')::date
            "#,
        )
        .fetch_all(pool)
        .await?;

        let mut sent = 0u64;
        for row in rows {
            let result = EmailService::send_payout_reminder_email(
                &config.resend_api_key,
                &config.resend_from_email,
                &row.email,
                &row.display_name,
                &row.group_name,
                &row.scheduled_date.format("%B %d, %Y").to_string(),
                &format_amount(row.amount),
            )
            .await;

            match result {
                Ok(()) => {
                    sqlx::query(
                        "UPDATE payout_schedule SET reminder_sent_at = NOW() WHERE id = $1",
                    )
                    .bind(row.id)
                    .execute(pool)
                    .await?;
                    sent += 1;
                }
                Err(e) => {
                    tracing::error!(error = %e, payout_schedule_id = %row.id, "Failed to send payout reminder");
                }
            }
        }

        if sent > 0 {
            tracing::info!(count = sent, "Payout reminders sent");
        }
        Ok(sent)
    }

    /// Emails the member at `position` in `group_id` that their payout has been recorded.
    /// Best-effort: errors are returned to the caller to log, not to fail the request.
    #[tracing::instrument(skip(pool, config))]
    pub async fn notify_payout_recipient(
        pool: &PgPool,
        config: &Config,
        group_id: Uuid,
        position: i32,
    ) -> AppResult<()> {
        let row = sqlx::query_as::<_, PayoutRecipientRow>(
            r#"
            SELECT u.display_name, u.email, g.name AS group_name, ps.amount
            FROM members m
            JOIN users u ON u.id = m.user_id
            JOIN groups g ON g.id = m.group_id
            LEFT JOIN payout_schedule ps
                ON ps.group_id = m.group_id AND ps.payout_round = m.payout_position
            WHERE m.group_id = $1 AND m.payout_position = $2
            "#,
        )
        .bind(group_id)
        .bind(position)
        .fetch_optional(pool)
        .await?;

        let Some(row) = row else {
            tracing::warn!(%group_id, position, "No member found for payout notification");
            return Ok(());
        };

        EmailService::send_payout_received_email(
            &config.resend_api_key,
            &config.resend_from_email,
            &row.email,
            &row.display_name,
            &row.group_name,
            &format_amount(row.amount),
        )
        .await
    }
}
