use crate::state::AppState;
use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, patch_with, post_with},
};

mod create;
mod delete;
mod dtos;
mod get;
mod list;
mod update_state;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::<AppState>::new()
        .api_route("/", get_with(list::handler, list::docs))
        .api_route("/", post_with(create::handler, create::docs))
        .api_route("/{id}", get_with(get::handler, get::docs))
        .api_route("/{id}", delete_with(delete::handler, delete::docs))
        .api_route(
            "/{id}/state",
            patch_with(update_state::handler, update_state::docs),
        )
}
