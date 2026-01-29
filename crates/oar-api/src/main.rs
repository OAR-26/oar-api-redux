use oar_domain::users::ports::UserRepository;
use oar_infrastructure::repositories::user_repo::PostgresUserRepository;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Initializing OAR API...");

    let user_repo = PostgresUserRepository::new();
    let app_state: Arc<dyn UserRepository> = Arc::new(user_repo);

    let app = handlers::app_router()
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("🚀 Server ready at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
