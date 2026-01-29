use schemars::JsonSchema;
use serde::Serialize;

// #[derive(Deserialize, JsonSchema)]
// pub struct RegisterRequest {
//     pub username: String,
//     pub email: String,
//     pub password: String,
// }

#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}
