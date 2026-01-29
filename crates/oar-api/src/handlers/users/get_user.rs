use super::dtos::UserResponse;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use oar_domain::users::ports::UserRepository;
use std::sync::Arc;

use aide::transform::TransformOperation;

pub async fn handler(
    Path(id): Path<uuid::Uuid>,
    State(repo): State<Arc<dyn UserRepository>>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = repo
        .find_by_id(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
    }))
}

pub fn docs(op: TransformOperation) -> TransformOperation {
    op.description("Get user")
        .response::<200, Json<UserResponse>>()
        .response::<404, ()>() // Tells Aide 404 is a possible exit
}
