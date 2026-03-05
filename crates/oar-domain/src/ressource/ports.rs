use super::{aggregate::Resource, value_objects::*};
use async_trait::async_trait;

pub struct NewResource {
    pub resource_type: String,
    pub network_address: String,
    pub cpuset: String,
    pub besteffort: bool,
    pub deploy: bool,
    pub desktop_computing: bool,
    pub available_upto: i64,
}

#[async_trait]
pub trait ResourceRepository: Send + Sync {
    async fn find_by_id(&self, id: &ResourceId) -> super::Result<Option<Resource>>;
    async fn find_by_state(&self, state: &ResourceState) -> super::Result<Vec<Resource>>;
    async fn find_by_network_address(&self, address: &str) -> super::Result<Vec<Resource>>;
    async fn find_all(&self) -> super::Result<Vec<Resource>>;
    async fn create(&self, resource: NewResource) -> super::Result<Resource>;
    async fn update_state(&self, id: &ResourceId, state: ResourceState) -> super::Result<()>;
    async fn update_next_state(
        &self,
        id: &ResourceId,
        next_state: ResourceNextState,
    ) -> super::Result<()>;
    async fn delete(&self, id: &ResourceId) -> super::Result<()>;
}

#[async_trait]
pub trait ResourceService: Send + Sync {
    // State transitions — finaud is the OAR fault detection system
    async fn mark_alive(&self, id: &ResourceId) -> super::Result<()>;
    async fn mark_dead(&self, id: &ResourceId) -> super::Result<()>;
    async fn mark_suspected(&self, id: &ResourceId, finaud: bool) -> super::Result<()>;
    async fn mark_absent(&self, id: &ResourceId, until: Option<i64>) -> super::Result<()>;
    async fn drain(&self, id: &ResourceId) -> super::Result<()>; // stop accepting new jobs
    async fn undrain(&self, id: &ResourceId) -> super::Result<()>;
    async fn apply_next_state(&self, id: &ResourceId) -> super::Result<()>; // transitions next_state → state
}
