use serde::{Deserialize, Serialize};

//  basic to get things started
#[derive(Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub exp: i64,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct ApiToken {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub token: String,
    pub name: String,
    pub created_at: i64,
}
