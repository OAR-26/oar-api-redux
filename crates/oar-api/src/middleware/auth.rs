use aide::operation::OperationIo;
use axum::{
    extract::{FromRef, FromRequestParts, Request},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};
use oar_domain::iam::ports::TokenService;
use std::sync::Arc;
use tracing::{error, info, warn};

#[derive(OperationIo)]
pub struct CurrentUser {
    pub user_id: uuid::Uuid,
    pub role: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    Arc<dyn TokenService>: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token_service = Arc::<dyn TokenService>::from_ref(state);

        let token = extract_bearer_token(parts.headers.get("authorization")).ok_or_else(|| {
            warn!("Missing or malformed authorization header");
            StatusCode::UNAUTHORIZED
        })?;

        let claims = token_service.verify_token(token).await.map_err(|e| {
            error!("Token verification failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        info!("Token verified successfully for user: {}", claims.sub);
        Ok(CurrentUser {
            user_id: claims.sub,
            role: claims.role,
        })
    }
}

/// Creates an authentication middleware factory that captures the token service
pub fn auth_middleware(
    token_service: Arc<dyn TokenService>,
) -> impl Fn(
    Request,
    Next,
)
    -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>>
+ Clone {
    move |req: Request, next: Next| {
        let token_service = token_service.clone();
        Box::pin(async move {
            let token =
                extract_bearer_token(req.headers().get("authorization")).ok_or_else(|| {
                    warn!("Missing or malformed authorization header in middleware");
                    StatusCode::UNAUTHORIZED
                })?;

            token_service.verify_token(token).await.map_err(|e| {
                error!("Token verification failed in middleware: {}", e);
                StatusCode::UNAUTHORIZED
            })?;

            info!("Token verified successfully in middleware");
            Ok(next.run(req).await)
        })
    }
}

fn extract_bearer_token(header: Option<&axum::http::HeaderValue>) -> Option<&str> {
    header
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
}
