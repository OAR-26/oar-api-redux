use super::dtos::{ApiKeyPath, CreateApiKeyRequest, CreateApiKeyResponse};
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{Json, extract::State, http::StatusCode};
use oar_domain::iam::ports::ApiKeyRepository;
use std::sync::Arc;
use tracing::{error, info};

pub async fn handler(
    current_user: CurrentUser,
    State(api_key_repo): State<Arc<dyn ApiKeyRepository>>,
    Json(body): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, StatusCode> {
    let raw_key = api_key_repo
        .create(current_user.user_id, body.name, body.role, body.expires_at)
        .await
        .map_err(|e| {
            error!("Failed to create API key: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("API key created for user: {}", current_user.user_id);
    Ok(Json(CreateApiKeyResponse { raw_key }))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Create API key")
        .description("Creates a new API key. The raw key is returned exactly once — store it securely, it cannot be retrieved again.")
        .tag("API Keys")
        .security_requirement("bearerAuth")
        .response::<201, Json<CreateApiKeyResponse>>()
        .response_with::<500, (), _>(|r| r.description("Internal server error"))
}
