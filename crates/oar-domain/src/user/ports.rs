use super::models::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, String>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, String>;
}
