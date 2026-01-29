use crate::config::Config;
use aide::openapi::{Info, OpenApi};
use axum::Extension;
use oar_domain::users::ports::UserRepository;
use oar_infrastructure::repositories::user_repo::PostgresUserRepository;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

mod config;
mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();

    tracing::info!(
        "Configuration loaded. Connecting to DB at: {}",
        config.database_url
    );

    let user_repo = PostgresUserRepository::new();
    let app_state: Arc<dyn UserRepository> = Arc::new(user_repo);

    let app = handlers::app_router()
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let mut api = OpenApi {
        info: Info {
            title: "OAR 3 API".to_string(),
            description: Some("description".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let addr = format!("{}:{}", config.api_host, config.api_port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("🚀 Server ready at http://{}", addr);
    tracing::info!("🔎 Deep dive into the api with Scalar http://{}/docs", addr);
    tracing::info!("😏 If you refer Swagger instead: http://{}/docs/swagger", addr);

    axum::serve(listener, app.finish_api(&mut api).layer(Extension(api)))
        .await
        .unwrap();
}
