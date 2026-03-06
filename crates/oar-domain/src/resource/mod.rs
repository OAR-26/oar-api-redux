mod aggregate;
mod entities;
pub mod ports;
pub mod value_objects;

pub use aggregate::Resource;
pub use entities::ResourceLog;
pub use value_objects::{ResourceId, ResourceNextState, ResourceState};

#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
    #[error("Resource {0:?} not found")]
    NotFound(ResourceId),

    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: ResourceState,
        to: ResourceState,
    },

    #[error("Resource {0:?} is draining and cannot accept this operation")]
    ResourceDraining(ResourceId),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type Result<T> = std::result::Result<T, ResourceError>;
