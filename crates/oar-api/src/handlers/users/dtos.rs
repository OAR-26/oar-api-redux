use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct UserPath {
    // the comment bellow will be picked up with swagger ;)
    /// The unique UUID of the user
    pub id: uuid::Uuid,
}

#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, JsonSchema)]
pub struct AuthResponse {
    pub token: String,
}
