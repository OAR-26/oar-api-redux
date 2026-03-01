use super::dtos::{UserPath, UserResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use oar_domain::user::ports::{UserRepository, PasswordService, TokenService};
use std::sync::Arc;

use aide::transform::TransformOperation;
use tracing::{info, error, warn};

pub async fn handler(
    Path(path): Path<UserPath>,
    State(state): State<(Arc<dyn UserRepository>, Arc<dyn PasswordService>, Arc<dyn TokenService>)>,
) -> Result<Json<UserResponse>, StatusCode> {
    let (user_repo, _, _) = state;
    
    info!("Fetching user with ID: {}", path.id);
    
    let user = user_repo
        .find_by_id(path.id)
        .await
        .map_err(|e| {
            error!("Database error while fetching user {}: {}", path.id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            warn!("User not found: {}", path.id);
            StatusCode::NOT_FOUND
        })?;

    info!("Successfully retrieved user: {} ({})", user.username, user.email);
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
