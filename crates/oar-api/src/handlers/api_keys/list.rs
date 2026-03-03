use super::dtos::ApiKeyResponse;
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{Json, extract::State, http::StatusCode};
use oar_domain::iam::ports::ApiKeyRepository;
use std::sync::Arc;
use tracing::{error, info};

pub async fn handler(
    current_user: CurrentUser,
    State(api_key_repo): State<Arc<dyn ApiKeyRepository>>,
) -> Result<Json<Vec<ApiKeyResponse>>, StatusCode> {
    let keys = api_key_repo
        .list_for_user(current_user.user_id)
        .await
        .map_err(|e| {
            error!(
                "Failed to list API keys for user {}: {}",
                current_user.user_id, e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(
        "Listed {} API keys for user {}",
        keys.len(),
        current_user.user_id
    );
    Ok(Json(
        keys.into_iter()
            .map(|k| ApiKeyResponse {
                id: k.id,
                name: k.name,
                role: k.role,
                expires_at: k.expires_at,
                created_at: k.created_at,
            })
            .collect(),
    ))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("List API keys")
        .description("Returns metadata for all API keys belonging to the authenticated user. Never includes key hashes.")
        .tag("API Keys")
        .security_requirement("bearerAuth")
        .response::<200, Json<Vec<ApiKeyResponse>>>()
        .response_with::<500, (), _>(|r| r.description("Internal server error"))
}
