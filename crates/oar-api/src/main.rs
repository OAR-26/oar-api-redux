use crate::config::Config;
use crate::state::AppState;
use aide::openapi::{Info, OpenApi, ReferenceOr};
use axum::Extension;
use indexmap::IndexMap;
use oar_domain::iam::ports::{PasswordService, TokenService};
use oar_domain::user::ports::UserRepository;
use oar_infrastructure::database::create_pool;
use oar_infrastructure::repositories::user_repo::PostgresUserRepository;
use oar_infrastructure::services::jwt_service::JwtServiceImpl;
use oar_infrastructure::services::password_service::Argon2PasswordService;
use std::sync::Arc;

use aide::openapi::SecurityScheme;

mod config;
mod handlers;
mod middleware;
mod state;

#[tokio::main]
async fn main() {
    // Configure logging for development mode
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "oar_api=debug,axum=debug,sqlx=debug");
        }
    }

    tracing_subscriber::fmt()
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .init();

    let config = Config::from_env();

    tracing::info!(
        "Configuration loaded. Connecting to DB at: {}",
        config.database_url
    );
    /////////////////////////////////////////////////////////////////////////////////////////////

    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    let app_state = AppState::new(
        Arc::new(PostgresUserRepository::new(pool)),
        Arc::new(Argon2PasswordService),
        Arc::new(JwtServiceImpl::new(
            config.jwt_secret.clone(),
            config.token_expiration_seconds,
        )),
    );

    /////////////////////////////////////////////////////////////////////////////////////////////
    let mut api = OpenApi {
        info: Info {
            title: "OAR 3 API".to_string(),
            description: Some("description".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let mut security_schemes: IndexMap<String, ReferenceOr<SecurityScheme>> = IndexMap::new();

    security_schemes.insert(
        "bearerAuth".to_string(),
        ReferenceOr::Item(SecurityScheme::Http {
            scheme: "bearer".to_string(),
            bearer_format: Some("JWT".to_string()),
            description: None,
            extensions: IndexMap::new(),
        }),
    );

    api.components
        .get_or_insert_with(Default::default)
        .security_schemes = security_schemes;

    let addr = format!("{}:{}", config.api_host, config.api_port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("🚀 Server ready at http://{}", addr);
    tracing::info!("🔎 Deep dive into the api with Scalar http://{}/docs", addr);
    tracing::info!(
        "😏 If you prefer Swagger instead: http://{}/docs/swagger",
        addr
    );

    let app = handlers::app_router(app_state);

    axum::serve(listener, app.finish_api(&mut api).layer(Extension(api)))
        .await
        .unwrap();
}
