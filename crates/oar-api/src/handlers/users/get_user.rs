use super::dtos::{UserPath, UserResponse};
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use oar_domain::user::ports::UserRepository;
use std::sync::Arc;
use tracing::{error, info, warn};

/// Handles requests to get user information by ID
pub async fn handler(
    current_user: CurrentUser,
    Path(path): Path<UserPath>,
    State(user_repo): State<Arc<dyn UserRepository>>,
) -> Result<Json<UserResponse>, StatusCode> {
    if current_user.user_id != path.id {
        warn!(
            "User {} attempted to access data for user {}",
            current_user.user_id, path.id
        );
        return Err(StatusCode::FORBIDDEN);
    }

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

    info!(
        "Successfully retrieved user: {} ({})",
        user.username, user.email
    );
    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    }))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Get user by ID")
        .description("Retrieve a detailed user profile by their unique UUID.")
        .tag("Users")
        .security_requirement("bearerAuth") //  must match the key you registered in security_schemes
        .response::<200, Json<UserResponse>>()
        .response_with::<401, (), _>(|res| res.description("Missing or invalid token"))
        .response_with::<403, (), _>(|res| res.description("Requesting another user's data"))
        .response_with::<404, (), _>(|res| res.description("User not found"))
        .response_with::<500, (), _>(|res| res.description("Internal server error"))
}
