use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "group_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GroupStatus {
    Pending,
    Active,
    Completed,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "contribution_frequency", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ContributionFrequency {
    Weekly,
    Biweekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub contribution_amount: rust_decimal::Decimal,
    pub frequency: ContributionFrequency,
    pub max_members: i32,
    pub current_members: i32,
    pub status: GroupStatus,
    pub start_date: NaiveDate,
    pub creator_id: Uuid,
    pub contract_group_id: Option<String>,
    pub current_payout_position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupInput {
    pub name: String,
    pub description: Option<String>,
    pub contribution_amount: rust_decimal::Decimal,
    pub frequency: ContributionFrequency,
    pub max_members: i32,
    pub start_date: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct ListGroupsQuery {
    pub cursor: Option<Uuid>,
    pub limit: Option<i64>,
    pub status: Option<GroupStatus>,
}
