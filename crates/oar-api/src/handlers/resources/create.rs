use super::dtos::{CreateResourceRequest, ResourceLogResponse, ResourceResponse};
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{Json, extract::State, http::StatusCode};
use oar_domain::resource::ports::{NewResource, ResourceRepository};
use std::sync::Arc;
use tracing::{error, info};

pub async fn handler(
    _current_user: CurrentUser,
    State(resource_repo): State<Arc<dyn ResourceRepository>>,
    Json(body): Json<CreateResourceRequest>,
) -> Result<Json<ResourceResponse>, StatusCode> {
    let resource = resource_repo
        .create(NewResource {
            resource_type: body.resource_type,
            network_address: body.network_address,
            cpuset: body.cpuset,
            besteffort: body.besteffort,
            deploy: body.deploy,
            desktop_computing: body.desktop_computing,
            available_upto: body.available_upto.unwrap_or(2147483647),
        })
        .await
        .map_err(|e| {
            error!("Failed to create resource: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Created resource {}", resource.id.0);
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
        logs: vec![],
    }))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("Create resource")
        .description("Register a new cluster resource.")
        .tag("Resources")
        .security_requirement("bearerAuth")
        .response::<200, Json<ResourceResponse>>()
        .response_with::<500, (), _>(|r| r.description("Internal server error"))
}
