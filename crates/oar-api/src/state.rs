use axum::extract::FromRef;
use oar_domain::{
    iam::ports::{PasswordService, TokenService},
    user::ports::UserRepository,
};
use std::sync::Arc;

/// Application state containing all service dependencies
#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub password_service: Arc<dyn PasswordService>,
    pub token_service: Arc<dyn TokenService>,
}

impl AppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_service: Arc<dyn PasswordService>,
        token_service: Arc<dyn TokenService>,
    ) -> Self {
        Self {
            user_repo,
            password_service,
            token_service,
        }
    }
}

// Teach axum how to pull each sub-component out of AppState
impl FromRef<AppState> for Arc<dyn UserRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.user_repo.clone()
    }
}

impl FromRef<AppState> for Arc<dyn PasswordService> {
    fn from_ref(state: &AppState) -> Self {
        state.password_service.clone()
    }
}

impl FromRef<AppState> for Arc<dyn TokenService> {
    fn from_ref(state: &AppState) -> Self {
        state.token_service.clone()
    }
}
