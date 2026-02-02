use super::dtos::{UserPath, UserResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use oar_domain::user::ports::UserRepository;
use std::sync::Arc;

use aide::transform::TransformOperation;

pub async fn handler(
    Path(path): Path<UserPath>,
    State(repo): State<Arc<dyn UserRepository>>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = repo
        .find_by_id(path.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    }))
}

// manual doc example
pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Get user by ID")
        .description("Retrieve a detailed user profile by their unique UUID from the database.")
        .tag("Users")
        .response::<200, Json<UserResponse>>()
        // Document possible errors
        .response_with::<404, (), _>(|res| res.description("User not found in the system"))
        .response_with::<500, (), _>(|res| {
            res.description("Internal server error - something went wrong on our end")
        })
}
