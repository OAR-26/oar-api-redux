use super::models::{User, Claims, ApiToken};
use crate::user::errors::AuthError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, String>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, String>;
}

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn generate_token(&self, user_id: uuid::Uuid) -> Result<String, AuthError>;
    async fn verify_token(&self, token: &str) -> Result<Claims, AuthError>;
}

#[async_trait]
pub trait PasswordService: Send + Sync {
    async fn hash_password(&self, plain: &str) -> Result<String, AuthError>;
    async fn verify_password(&self, plain: &str, hash: &str) -> Result<bool, AuthError>;
}
