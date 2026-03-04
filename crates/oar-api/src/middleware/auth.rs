use aide::operation::OperationIo;
use axum::{
    extract::{FromRef, FromRequestParts, Request},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};
use oar_domain::iam::{models::AuthCredential, ports::AuthService};
use std::sync::Arc;
use tracing::{info, warn};

use std::future::Future;
use std::pin::Pin;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

#[derive(OperationIo)]
pub struct CurrentUser {
    pub user_id: uuid::Uuid,
    pub role: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    Arc<dyn AuthService>: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, StatusCode> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                warn!("Missing authorization header");
                StatusCode::UNAUTHORIZED
            })?;

        let credential = parse_auth_header(auth_header)?;

        Arc::<dyn AuthService>::from_ref(state)
            .authenticate(credential)
            .await
            .map(|identity| {
                info!("Authenticated user: {}", identity.user_id);
                CurrentUser {
                    user_id: identity.user_id,
                    role: identity.role,
                }
            })
            .map_err(|e| {
                warn!("Authentication failed: {}", e);
                StatusCode::UNAUTHORIZED
            })
    }
}

pub fn auth_middleware(
    auth_service: Arc<dyn AuthService>,
) -> impl Fn(Request, Next) -> BoxFuture<Result<Response, StatusCode>> + Clone {
    move |req: Request, next: Next| {
        let auth_service = auth_service.clone();

        Box::pin(async move {
            let auth_header = req
                .headers()
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .ok_or_else(|| {
                    warn!("Missing authorization header in middleware");
                    StatusCode::UNAUTHORIZED
                })?;

            let credential = parse_auth_header(auth_header)?;

            auth_service.authenticate(credential).await.map_err(|e| {
                warn!("Authentication failed in middleware: {}", e);
                StatusCode::UNAUTHORIZED
            })?;

            info!("Request authenticated in middleware");

            Ok(next.run(req).await)
        }) as BoxFuture<Result<Response, StatusCode>>
    }
}
fn parse_auth_header(header: &str) -> Result<AuthCredential, StatusCode> {
    if let Some(token) = header.strip_prefix("Bearer ") {
        return Ok(AuthCredential::BearerToken(token.to_string()));
    }

    if let Some(key) = header.strip_prefix("ApiKey ") {
        return Ok(AuthCredential::ApiKey(key.to_string()));
    }

    warn!("Invalid authorization scheme");
    Err(StatusCode::UNAUTHORIZED)
}
