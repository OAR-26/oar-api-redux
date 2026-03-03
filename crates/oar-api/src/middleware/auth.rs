use aide::operation::OperationIo;
use axum::{
    extract::{FromRef, FromRequestParts, Request},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use oar_domain::iam::ports::ApiKeyRepository; // ← add
use oar_domain::iam::ports::TokenService;
use sha2::{Digest, Sha256}; // ← add
use std::sync::Arc;
use tracing::{error, info, warn};

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
    Arc<dyn TokenService>: FromRef<S>,
    Arc<dyn ApiKeyRepository>: FromRef<S>,
{
    type Rejection = StatusCode; // ← was missing

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, StatusCode> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok());

        match auth_header {
            Some(h) if h.starts_with("Bearer ") => {
                let token_service = Arc::<dyn TokenService>::from_ref(state);
                let token = &h[7..];
                let claims = token_service.verify_token(token).await.map_err(|e| {
                    error!("JWT verification failed: {}", e);
                    StatusCode::UNAUTHORIZED
                })?;
                info!("JWT authenticated user: {}", claims.sub);
                Ok(CurrentUser {
                    user_id: claims.sub,
                    role: claims.role,
                })
            }
            Some(h) if h.starts_with("ApiKey ") => {
                let api_key_repo = Arc::<dyn ApiKeyRepository>::from_ref(state);
                let raw_key = &h[7..];
                let hash = sha256(raw_key);
                let key = api_key_repo
                    .find_by_hash(&hash)
                    .await
                    .map_err(|e| {
                        error!("API key lookup failed: {}", e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?
                    .ok_or_else(|| {
                        warn!("API key not found");
                        StatusCode::UNAUTHORIZED
                    })?;

                if key.expires_at.map(|e| e < Utc::now()).unwrap_or(false) {
                    warn!("Expired API key used for user: {}", key.user_id);
                    return Err(StatusCode::UNAUTHORIZED);
                }

                info!("API key authenticated user: {}", key.user_id);
                Ok(CurrentUser {
                    user_id: key.user_id,
                    role: key.role,
                })
            }
            _ => {
                warn!("Missing or unrecognized authorization scheme");
                Err(StatusCode::UNAUTHORIZED)
            }
        }
    }
}

// auth_middleware is only used as a route_layer fallback —
// CurrentUser already handles both schemes for handlers directly.
// This middleware guards routes that don't use CurrentUser as an extractor.
pub fn auth_middleware(
    token_service: Arc<dyn TokenService>,
    api_key_repo: Arc<dyn ApiKeyRepository>, 
) -> impl Fn(Request, Next) -> BoxFuture<Result<Response, StatusCode>> + Clone {
    move |req: Request, next: Next| {
        let token_service = token_service.clone();
        let api_key_repo = api_key_repo.clone();
        Box::pin(async move {
            let auth_header = req
                .headers()
                .get("authorization")
                .and_then(|v| v.to_str().ok());

            match auth_header {
                Some(h) if h.starts_with("Bearer ") => {
                    let token = &h[7..];
                    token_service.verify_token(token).await.map_err(|e| {
                        error!("JWT verification failed in middleware: {}", e);
                        StatusCode::UNAUTHORIZED
                    })?;
                    info!("JWT verified in middleware");
                }
                Some(h) if h.starts_with("ApiKey ") => {
                    let hash = sha256(&h[7..]);
                    let key = api_key_repo
                        .find_by_hash(&hash)
                        .await
                        .map_err(|e| {
                            error!("API key lookup failed in middleware: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?
                        .ok_or_else(|| {
                            warn!("Invalid API key in middleware");
                            StatusCode::UNAUTHORIZED
                        })?;

                    if key.expires_at.map(|e| e < Utc::now()).unwrap_or(false) {
                        warn!("Expired API key in middleware");
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                    info!("API key verified in middleware");
                }
                _ => {
                    warn!("Missing or unrecognized authorization scheme in middleware");
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }

            Ok(next.run(req).await)
        }) as BoxFuture<Result<Response, StatusCode>>
    }
}

fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

fn extract_bearer_token(header: Option<&axum::http::HeaderValue>) -> Option<&str> {
    header
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
}
