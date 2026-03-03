use crate::state::AppState;
use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with},
};

mod create;
mod dtos;
mod list;
mod revoke;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::<AppState>::new() // ← explicit type, no more inference ambiguity
        .api_route("/", post_with(create::handler, create::docs))
        .api_route("/", get_with(list::handler, list::docs))
        .api_route("/{id}", delete_with(revoke::handler, revoke::docs))
}
