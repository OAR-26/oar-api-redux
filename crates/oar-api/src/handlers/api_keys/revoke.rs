use super::dtos::ApiKeyPath;
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use oar_domain::iam::{errors::AuthError, ports::ApiKeyRepository};
use std::sync::Arc;
use tracing::{error, info, warn};

pub async fn handler(
    current_user: CurrentUser,
    Path(path): Path<ApiKeyPath>,
    State(api_key_repo): State<Arc<dyn ApiKeyRepository>>,
) -> Result<StatusCode, StatusCode> {
    api_key_repo
        .revoke(path.id, current_user.user_id)
        .await
        .map_err(|e| match e {
            AuthError::ApiKeyNotFound => {
                warn!(
                    "API key {} not found or not owned by user {}",
                    path.id, current_user.user_id
                );
                StatusCode::NOT_FOUND
            }
            _ => {
                error!("Failed to revoke API key {}: {}", path.id, e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    info!(
        "API key {} revoked by user {}",
        path.id, current_user.user_id
    );
    Ok(StatusCode::NO_CONTENT)
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Revoke API key")
        .description("Permanently revokes an API key. Users can only revoke their own keys.")
        .tag("API Keys")
        .security_requirement("bearerAuth")
        .response_with::<204, (), _>(|r| r.description("Key successfully revoked"))
        .response_with::<404, (), _>(|r| r.description("Key not found or not owned by this user"))
        .response_with::<500, (), _>(|r| r.description("Internal server error"))
}
