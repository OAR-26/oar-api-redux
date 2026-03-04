use axum::extract::FromRef;
use oar_domain::{
    iam::ports::{ApiKeyRepository, AuthService},
    user::ports::UserRepository,
};
use std::sync::Arc;

/// Application state containing all service dependencies
#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub api_key_repo: Arc<dyn ApiKeyRepository>,
    pub auth_service: Arc<dyn AuthService>,
}

impl AppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        api_key_repo: Arc<dyn ApiKeyRepository>,
        auth_service: Arc<dyn AuthService>,
    ) -> Self {
        Self {
            user_repo,
            api_key_repo,
            auth_service,
        }
    }
}

// Teach axum how to pull each sub-component out of AppState
impl FromRef<AppState> for Arc<dyn UserRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.user_repo.clone()
    }
}

impl FromRef<AppState> for Arc<dyn ApiKeyRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.api_key_repo.clone()
    }
}

impl FromRef<AppState> for Arc<dyn AuthService> {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}
