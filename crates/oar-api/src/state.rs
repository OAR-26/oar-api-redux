use axum::extract::FromRef;
use oar_domain::user::ports::{PasswordService, TokenService, UserRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub password_service: Arc<dyn PasswordService>,
    pub token_service: Arc<dyn TokenService>,
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