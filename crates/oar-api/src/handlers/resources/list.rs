use super::dtos::{ResourceLogResponse, ResourceResponse, ResourceStateQuery};
use crate::middleware::auth::CurrentUser;
use aide::transform::TransformOperation;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use oar_domain::resource::ports::ResourceRepository;
use std::sync::Arc;
use tracing::{error, info};

pub async fn handler(
    _current_user: CurrentUser,
    Query(query): Query<ResourceStateQuery>,
    State(resource_repo): State<Arc<dyn ResourceRepository>>,
) -> Result<Json<Vec<ResourceResponse>>, StatusCode> {
    let resources = match query.network_address {
        Some(addr) => resource_repo.find_by_network_address(&addr).await,
        None => resource_repo.find_all().await,
    }
    .map_err(|e| {
        error!("Failed to list resources: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!("Listed {} resources", resources.len());
    Ok(Json(
        resources
            .into_iter()
            .map(|r| ResourceResponse {
                id: r.id.0,
                resource_type: r.resource_type,
                network_address: r.network_address,
                state: format!("{:?}", r.state),
                next_state: format!("{:?}", r.next_state),
                suspended_jobs: r.suspended_jobs,
                scheduler_priority: r.scheduler_priority,
                cpuset: r.cpuset,
                besteffort: r.besteffort,
                deploy: r.deploy,
                drain: r.drain,
                available_upto: r.available_upto,
                logs: r
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
            })
            .collect(),
    ))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.summary("List resources")
        .description("Returns all cluster resources, optionally filtered by network address.")
        .tag("Resources")
        .security_requirement("bearerAuth")
        .response::<200, Json<Vec<ResourceResponse>>>()
        .response_with::<500, (), _>(|r| r.description("Internal server error"))
}
