use crate::state::AppState;
use aide::{
    axum::{ApiRouter, IntoApiResponse, routing::get},
    openapi::OpenApi,
    scalar::Scalar,
    swagger::Swagger,
};
use axum::{Extension, Json};
use tower_http::trace::TraceLayer;
mod users;

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

pub fn app_router(app_state: AppState) -> ApiRouter {
    ApiRouter::new()
        .route("/docs", Scalar::new("/api.json").axum_route())
        .route("/docs/swagger", Swagger::new("/api.json").axum_route())
        .route("/api.json", get(serve_api))
        .nest("/users", users::router(&app_state))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
