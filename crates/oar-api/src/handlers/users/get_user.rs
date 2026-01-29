use super::dtos::UserResponse;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use oar_domain::users::ports::UserRepository;
use std::sync::Arc;

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
