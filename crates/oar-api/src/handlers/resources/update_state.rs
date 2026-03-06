use super::dtos::{ResourcePath, UpdateStateRequest};
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use oar_domain::resource::{
    ports::ResourceRepository,
    value_objects::{ResourceId, ResourceState},
};
use std::sync::Arc;
use tracing::{error, info, warn};

pub async fn handler(
    _current_user: CurrentUser,
    Path(path): Path<ResourcePath>,
    State(resource_repo): State<Arc<dyn ResourceRepository>>,
    Json(body): Json<UpdateStateRequest>,
) -> Result<StatusCode, StatusCode> {
    let state = match body.state.as_str() {
        "Alive" => ResourceState::Alive,
        "Dead" => ResourceState::Dead,
        "Suspected" => ResourceState::Suspected,
        "Absent" => ResourceState::Absent,
        _ => {
            warn!("Invalid resource state: {}", body.state);
            return Err(StatusCode::UNPROCESSABLE_ENTITY);
        }
    };

    resource_repo
        .update_state(&ResourceId(path.id), state)
        .await
        .map_err(|e| {
            error!("Failed to update resource {} state: {}", path.id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Updated resource {} state to {}", path.id, body.state);
    Ok(StatusCode::NO_CONTENT)
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Update resource state")
        .description(
            "Transition a resource to a new state. Valid values: Alive, Dead, Suspected, Absent.",
        )
        .tag("Resources")
        .security_requirement("bearerAuth")
        .response_with::<204, (), _>(|r| r.description("State updated"))
        .response_with::<404, (), _>(|r| r.description("Resource not found"))
        .response_with::<422, (), _>(|r| r.description("Invalid state value"))
        .response_with::<500, (), _>(|r| r.description("Internal server error"))
}
