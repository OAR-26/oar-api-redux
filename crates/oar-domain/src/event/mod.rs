mod aggregate;
mod entities;
pub mod ports;
mod value_objects;

pub use aggregate::Event;
pub use entities::EventHostname;
pub use value_objects::{EventId, ToCheck};

#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error("Event {0:?} not found")]
    NotFound(EventId),

    #[error("No events found for job {0}")]
    NoEventsForJob(i64),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type Result<T> = std::result::Result<T, EventError>;
