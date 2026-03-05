mod aggregate;
mod entities;
pub mod ports;
mod value_objects;

pub use aggregate::Job;
pub use entities::{
    Challenge, FragJob, JobDependency, JobStateLog, MoldableDescription, WalltimeChange,
};
pub use value_objects::{DependencyIndex, FragState, JobId, JobState, JobType, ReservationState};

#[derive(Debug, thiserror::Error)]
pub enum JobError {
    #[error("Job {0:?} not found")]
    NotFound(JobId),

    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition { from: JobState, to: JobState },

    #[error("Job {0:?} is not in a state to accept this operation")]
    InvalidStateForOperation(JobId),

    #[error("Moldable description {0} not found")]
    DescriptionNotFound(i64),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type Result<T> = std::result::Result<T, JobError>;
