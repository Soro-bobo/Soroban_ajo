use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "contribution_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ContributionStatus {
    Pending,
    Confirmed,
    Failed,
    Missed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Contribution {
    pub id: Uuid,
    pub group_id: Uuid,
    pub member_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub tx_hash: String,
    pub status: ContributionStatus,
    pub period_date: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateContributionInput {
    pub group_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub tx_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct ListContributionsQuery {
    pub cursor: Option<Uuid>,
    pub limit: Option<i64>,
    pub member_id: Option<Uuid>,
    pub status: Option<ContributionStatus>,
}
