use axum::Router;
use oar_domain::users::ports::UserRepository;
use std::sync::Arc;

mod users;

pub fn app_router() -> Router<Arc<dyn UserRepository>> {
    Router::new().nest("/users", users::router())
    // .nest("/other-ressource", ressource::router())
}
