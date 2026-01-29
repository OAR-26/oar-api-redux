use aide::{
    axum::{
        ApiRouter, IntoApiResponse,
        routing::{get, post},
    },
    openapi::{Info, OpenApi},
    swagger::Swagger,
};
use axum::{Extension, Json};
use oar_domain::users::ports::UserRepository;
use std::sync::Arc;
mod users;

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
pub fn app_router() -> ApiRouter<Arc<dyn UserRepository>> {
    ApiRouter::new()
        .route("/swagger", Swagger::new("/api.json").axum_route())
        .route("/api.json", get(serve_api))
        .nest("/users", users::router())

}
