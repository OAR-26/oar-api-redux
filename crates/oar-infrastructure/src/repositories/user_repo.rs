use async_trait::async_trait;
use oar_domain::users::{models::User, ports::UserRepository};
use uuid::Uuid;
pub struct PostgresUserRepository {
    // should be a real db pool;
}

impl PostgresUserRepository {
    pub fn new() -> Self {
        // db logic
        Self {}
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, user: User) -> Result<User, String> {
        println!("Saving user to Postgres: {}", user.email);
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        Ok(Some(User {
            id,
            email: "hello@example.com".to_string(),
            username: "amine".to_string(),
            password_hash: "hashed_stuff".to_string(),
        }))
    }
}
