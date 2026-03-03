use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct ApiKey {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub key_hash: String,
    pub name: String, // "service name"
    pub role: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub exp: i64,
    pub role: String,
}
