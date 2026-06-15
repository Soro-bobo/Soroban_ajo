use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "member_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MemberStatus {
    Pending,
    Active,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Member {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub payout_position: i32,
    pub status: MemberStatus,
    pub has_received_payout: bool,
    pub joined_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MemberWithUser {
    pub id: Uuid,
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub display_name: String,
    pub email: String,
    pub wallet_address: Option<String>,
    pub payout_position: i32,
    pub status: MemberStatus,
    pub has_received_payout: bool,
    pub joined_at: DateTime<Utc>,
}
