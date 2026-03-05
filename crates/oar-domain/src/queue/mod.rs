mod aggregate;
pub mod ports;
mod value_objects;

pub use aggregate::Queue;
pub use value_objects::{QueueName, QueueState};

#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("Queue {0:?} not found")]
    NotFound(QueueName),

    #[error("Queue {0:?} already exists")]
    AlreadyExists(QueueName),

    #[error("Invalid scheduler policy: {0}")]
    InvalidSchedulerPolicy(String),

    #[error("Cannot delete queue {0:?} — it still has jobs")]
    QueueNotEmpty(QueueName),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type Result<T> = std::result::Result<T, QueueError>;
