use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Response,
};
use oar_domain::user::ports::TokenService;
use oar_domain::user::models::Claims;
use std::sync::Arc;
use tracing::{info, error, warn};

pub struct CurrentUser {
    pub user_id: uuid::Uuid,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    Arc<dyn TokenService>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token_service = Arc::<dyn TokenService>::from_ref(state);

        let auth_header = parts.headers
            .get("authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| {
                warn!("Missing authorization header");
                StatusCode::UNAUTHORIZED
            })?;

        if !auth_header.starts_with("Bearer ") {
            warn!("Invalid authorization header format: {}", auth_header);
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = &auth_header[7..];
        info!("Verifying token for authentication");
        
        let claims = token_service.verify_token(token).await
            .map_err(|e| {
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

pub async fn auth_middleware<B>(
    State(token_service): State<Arc<dyn TokenService>>,
    req: axum::extract::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            warn!("Missing authorization header in middleware");
            StatusCode::UNAUTHORIZED
        })?;

    if !auth_header.starts_with("Bearer ") {
        warn!("Invalid authorization header format in middleware: {}", auth_header);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    info!("Verifying token in middleware");
    
    let _claims = token_service.verify_token(token).await
        .map_err(|e| {
            error!("Token verification failed in middleware: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

    info!("Token verified successfully in middleware");
    Ok(next.run(req).await)
}
