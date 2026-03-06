use super::dtos::{ResourceLogResponse, ResourcePath, ResourceResponse};
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{
    Json,
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
) -> Result<Json<ResourceResponse>, StatusCode> {
    let resource = resource_repo
        .find_by_id(&ResourceId(path.id))
        .await
        .map_err(|e| {
            error!("Failed to fetch resource {}: {}", path.id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            warn!("Resource {} not found", path.id);
            StatusCode::NOT_FOUND
        })?;

    info!("Fetched resource {}", path.id);
    Ok(Json(ResourceResponse {
        id: resource.id.0,
        resource_type: resource.resource_type,
        network_address: resource.network_address,
        state: format!("{:?}", resource.state),
        next_state: format!("{:?}", resource.next_state),
        suspended_jobs: resource.suspended_jobs,
        scheduler_priority: resource.scheduler_priority,
        cpuset: resource.cpuset,
        besteffort: resource.besteffort,
        deploy: resource.deploy,
        drain: resource.drain,
        available_upto: resource.available_upto,
        logs: resource
            .logs
            .into_iter()
            .map(|l| ResourceLogResponse {
                id: l.id,
                attribute: l.attribute,
                value: l.value,
                date_start: l.date_start,
                date_stop: l.date_stop,
                finaud_decision: l.finaud_decision,
            })
            .collect(),
    }))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Get resource")
        .description("Fetch a single cluster resource by ID.")
        .tag("Resources")
        .security_requirement("bearerAuth")
        .response::<200, Json<ResourceResponse>>()
        .response_with::<404, (), _>(|r| r.description("Resource not found"))
        .response_with::<500, (), _>(|r| r.description("Internal server error"))
}
