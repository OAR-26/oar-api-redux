use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct UserPath {
    // this will be picked up with swagger ;)
    /// The unique UUID of the user
    pub id: uuid::Uuid,
}

#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}
