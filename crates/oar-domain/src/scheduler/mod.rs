mod aggregate;
pub mod ports;
mod value_objects;

pub use aggregate::Scheduler;
pub use ports::ScriptStatus;
pub use value_objects::SchedulerName;

#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("Scheduler {0:?} not found")]
    NotFound(SchedulerName),

    #[error("Scheduler {0:?} already exists")]
    AlreadyExists(SchedulerName),

    #[error("Script path is invalid or not executable: {0}")]
    InvalidScript(String),

    #[error("Cannot delete scheduler {0:?} — it is assigned to one or more queues")]
    SchedulerInUse(SchedulerName),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type Result<T> = std::result::Result<T, SchedulerError>;
