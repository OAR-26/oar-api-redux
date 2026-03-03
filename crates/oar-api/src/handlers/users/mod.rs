use crate::middleware::auth::auth_middleware;
use crate::state::AppState;
use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with},
};
use axum::middleware;

pub mod dtos;
mod get_user;
mod login;
mod register;

pub fn router(state: &AppState) -> ApiRouter<AppState> {
    // Routes that require auth
    let protected = ApiRouter::new()
        .api_route("/{id}", get_with(get_user::handler, get_user::docs))
        .route_layer(middleware::from_fn(
            auth_middleware(state.token_service.clone()), // ← captured here, no State needed
        ));

    // Routes that don't require auth
    let public = ApiRouter::new()
        .api_route("/login", post_with(login::handler, login::docs))
        .api_route("/register", post_with(register::handler, register::docs));

    ApiRouter::new().merge(protected).merge(public)
}
