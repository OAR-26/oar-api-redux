use axum::{
    Router,
    routing::{get, post},
};
use oar_domain::users::ports::UserRepository;
use std::sync::Arc;

pub mod dtos;
mod get_user;
mod login;
mod register;

pub fn router() -> Router<Arc<dyn UserRepository>> {
    Router::new()
        .route("/", post(register::handler))
        .route("/{id}", get(get_user::handler))
}
