use super::dtos::RegisterRequest;
use axum::{Json, extract::State, http::StatusCode};
use oar_domain::users::ports::UserRepository;
use std::sync::Arc;

pub async fn handler(
    State(repo): State<Arc<dyn UserRepository>>,
    Json(payload): Json<RegisterRequest>,
) -> StatusCode {
    // Logic here...
    StatusCode::CREATED
}
