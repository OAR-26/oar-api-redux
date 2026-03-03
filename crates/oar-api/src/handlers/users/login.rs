use super::dtos::{AuthResponse, LoginRequest};
use crate::state::AppState;
use aide::transform::TransformOperation;
use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, info, warn};

/// Documents the login endpoint for OpenAPI
pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Login").tag("Public")
}

/// Handles user login requests
pub async fn handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let AppState {
        user_repo,
        password_service,
        token_service,
    } = state;

    info!("Login attempt for email: {}", payload.email);

    let user = user_repo
        .find_by_email(&payload.email)
        .await
        .map_err(|e| {
            error!("Database error while finding user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            warn!("Login attempt for non-existent user: {}", payload.email);
            StatusCode::UNAUTHORIZED
        })?;

    // Verify password
    let password_valid = password_service
        .verify_password(&payload.password, &user.password_hash)
        .await
        .map_err(|e| {
            error!("Password verification error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !password_valid {
        warn!("Invalid password for user: {}", payload.email);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT token
    let token = token_service.generate_token(user.id).await.map_err(|e| {
        error!("Token generation error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!("Successful login for user: {}", payload.email);
    Ok(Json(AuthResponse { token }))
}
