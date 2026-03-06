use super::dtos::ResourcePath;
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use oar_domain::resource::{ports::ResourceRepository, value_objects::ResourceId};
use std::sync::Arc;
use tracing::{error, info, warn};

pub async fn handler(
    _current_user: CurrentUser,
    Path(path): Path<ResourcePath>,
    State(resource_repo): State<Arc<dyn ResourceRepository>>,
) -> Result<StatusCode, StatusCode> {
    resource_repo
        .delete(&ResourceId(path.id))
        .await
        .map_err(|e| match e {
            oar_domain::resource::ResourceError::NotFound(_) => {
                warn!("Resource {} not found for deletion", path.id);
                StatusCode::NOT_FOUND
            }
            _ => {
                error!("Failed to delete resource {}: {}", path.id, e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    info!("Deleted resource {}", path.id);
    Ok(StatusCode::NO_CONTENT)
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Delete resource")
        .description("Remove a cluster resource. This is irreversible.")
        .tag("Resources")
        .security_requirement("bearerAuth")
        .response_with::<204, (), _>(|r| r.description("Resource deleted"))
        .response_with::<404, (), _>(|r| r.description("Resource not found"))
        .response_with::<500, (), _>(|r| r.description("Internal server error"))
}
