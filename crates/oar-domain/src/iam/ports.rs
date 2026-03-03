// #[async_trait]
// pub trait ApiKeyRepository: Send + Sync {
//     async fn find_by_hash(&self, hash: &str) -> Result<Option<ApiKey>, ...>;
//     async fn create(&self, user_id: Uuid, name: String, role: String) -> Result<String, ...>; // returns raw key once
//     async fn revoke(&self, id: Uuid) -> Result<(), ...>;
// }

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::iam::{
    errors::AuthError,
    models::{ApiKey, Claims},
};

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn generate_token(&self, user_id: Uuid) -> Result<String, AuthError>;
    async fn verify_token(&self, token: &str) -> Result<Claims, AuthError>;
}

#[async_trait]
pub trait PasswordService: Send + Sync {
    async fn hash_password(&self, plain: &str) -> Result<String, AuthError>;
    async fn verify_password(&self, plain: &str, hash: &str) -> Result<bool, AuthError>;
}

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn find_by_hash(&self, hash: &str) -> Result<Option<ApiKey>, AuthError>;

    async fn create(
        &self,
        user_id: Uuid,
        name: String,
        role: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<String, AuthError>;
    async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<ApiKey>, AuthError>;

    async fn revoke(&self, id: Uuid, user_id: Uuid) -> Result<(), AuthError>;
}
