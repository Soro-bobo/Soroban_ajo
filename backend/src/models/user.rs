use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: String,
    pub wallet_address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub wallet_address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            display_name: u.display_name,
            wallet_address: u.wallet_address,
            is_active: u.is_active,
            created_at: u.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub wallet_address: Option<String>,
}
