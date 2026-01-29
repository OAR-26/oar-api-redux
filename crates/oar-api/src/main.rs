use oar_domain::users::ports::UserRepository;
use oar_infrastructure::repositories::user_repo::PostgresUserRepository;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::config::Config;

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

    let user_repo = oar_infrastructure::repositories::user_repo::PostgresUserRepository::new();
    let app_state: Arc<dyn UserRepository> = Arc::new(user_repo);

    let app = handlers::app_router()
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = format!("{}:{}", config.api_host, config.api_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("🚀 Server ready at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
