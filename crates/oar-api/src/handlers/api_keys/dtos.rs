use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, JsonSchema)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub role: String,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Returned once on creation — user must save this, it will never be shown again
#[derive(Serialize, JsonSchema)]
pub struct CreateApiKeyResponse {
    pub raw_key: String, // only time this ever appears in a response
}

/// Returned by list — never includes the hash
#[derive(Serialize, JsonSchema)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ApiKeyPath {
    pub id: Uuid,
}
