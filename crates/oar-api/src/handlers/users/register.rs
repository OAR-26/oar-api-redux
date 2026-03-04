use super::dtos::{RegisterRequest, UserResponse};
use crate::state::AppState;
use aide::transform::TransformOperation;
use axum::{Json, extract::State, http::StatusCode};
use oar_domain::user::models::User;
use tracing::{error, info, warn};

/// Documents the register endpoint for OpenAPI
pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Register new user").tag("Public")
}

/// Handles user registration requests
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let AppState {
        user_repo,
        auth_service,
        ..
    } = state;

    info!("Registration attempt for email: {}", payload.email);

    let existing_user = user_repo.find_by_email(&payload.email).await.map_err(|e| {
        error!("Database error while checking existing user: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if existing_user.is_some() {
        warn!("Registration attempt for existing email: {}", payload.email);
        return Err(StatusCode::CONFLICT);
    }

    // Hash password
    let password_hash = auth_service
        .hash_password(&payload.password)
        .await
        .map_err(|e| {
            error!("Password hashing failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create new user
    let new_user = User {
        id: uuid::Uuid::new_v4(),
        email: payload.email.clone(),
        username: payload.username.clone(),
        password_hash,
    };

    let created_user = user_repo.create_user(new_user).await.map_err(|e| {
        error!("Failed to create user in database: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!(
        "Successfully registered user: {} ({})",
        created_user.username, created_user.email
    );

    Ok(Json(UserResponse {
        id: created_user.id,
        username: created_user.username,
        email: created_user.email,
    }))
}
