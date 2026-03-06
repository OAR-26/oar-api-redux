use axum::extract::FromRef;
use oar_domain::iam::ports::{ApiKeyRepository, AuthService};
use oar_domain::resource::ports::ResourceRepository;
use oar_domain::user::ports::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub api_key_repo: Arc<dyn ApiKeyRepository>,
    pub auth_service: Arc<dyn AuthService>,
    pub resource_repo: Arc<dyn ResourceRepository>, // ← add
}

impl AppState {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        api_key_repo: Arc<dyn ApiKeyRepository>,
        auth_service: Arc<dyn AuthService>,
        resource_repo: Arc<dyn ResourceRepository>,
    ) -> Self {
        Self {
            user_repo,
            api_key_repo,
            auth_service,
            resource_repo,
        }
    }
}

impl FromRef<AppState> for Arc<dyn UserRepository> {
    fn from_ref(s: &AppState) -> Self {
        s.user_repo.clone()
    }
}
impl FromRef<AppState> for Arc<dyn ApiKeyRepository> {
    fn from_ref(s: &AppState) -> Self {
        s.api_key_repo.clone()
    }
}
impl FromRef<AppState> for Arc<dyn AuthService> {
    fn from_ref(s: &AppState) -> Self {
        s.auth_service.clone()
    }
}
impl FromRef<AppState> for Arc<dyn ResourceRepository> {
    fn from_ref(s: &AppState) -> Self {
        s.resource_repo.clone()
    }
}
