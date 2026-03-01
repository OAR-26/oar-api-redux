use crate::config::Config;
use aide::openapi::{Info, OpenApi};
use axum::Extension;
use oar_domain::user::ports::{PasswordService, TokenService, UserRepository};
use oar_infrastructure::repositories::user_repo::PostgresUserRepository;
use oar_infrastructure::services::jwt_service::Argon2PasswordService;
use oar_infrastructure::services::password_service::JwtServiceImpl;
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
    let user_repo_state: Arc<dyn UserRepository> = Arc::new(user_repo);

    let jwt_service =
        JwtServiceImpl::new(config.jwt_secret.clone(), config.token_expiration_seconds);
    let jwt_service_state: Arc<dyn TokenService> = Arc::new(jwt_service);

    let password_service = Argon2PasswordService;
    let password_service_state: Arc<dyn PasswordService> = Arc::new(password_service);

    let app = handlers::app_router()
        .layer(TraceLayer::new_for_http())
        .with_state((user_repo_state, password_service_state, jwt_service_state));

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
    tracing::info!(
        "😏 If you refer Swagger instead: http://{}/docs/swagger",
        addr
    );

    axum::serve(listener, app.finish_api(&mut api).layer(Extension(api)))
        .await
        .unwrap();
}
