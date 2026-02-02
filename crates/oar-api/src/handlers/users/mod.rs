use aide::axum::{ApiRouter, routing::get_with};
use oar_domain::user::ports::UserRepository;
use std::sync::Arc;

pub mod dtos;
mod get_user;
mod login;
mod register;

pub fn router() -> ApiRouter<Arc<dyn UserRepository>> {
    ApiRouter::new()
        // .route("/", post(register::handler))
        .api_route("/{id}", get_with(get_user::handler, get_user::docs))
}
