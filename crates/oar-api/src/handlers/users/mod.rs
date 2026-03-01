use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with},
};
use oar_domain::user::ports::{PasswordService, TokenService, UserRepository};
use std::sync::Arc;

pub mod dtos;
mod get_user;
mod login;
mod register;

pub fn router() -> ApiRouter<(
    Arc<dyn UserRepository>,
    Arc<dyn PasswordService>,
    Arc<dyn TokenService>,
)> {
    ApiRouter::new()
        .api_route("/login", post_with(login::handler, login::docs))
        .api_route("/{id}", get_with(get_user::handler, get_user::docs))
}
