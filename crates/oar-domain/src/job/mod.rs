mod aggregate;
mod entities;
pub mod ports;
mod value_objects; // Public so infrastructure can implement the trait

// This allows other modules to call `job::Job` instead of `job::aggregate::Job`
pub use aggregate::Job;
pub use value_objects::{JobId, JobState, JobType};

// Re-export only relevant Entities
// We often keep entities internal, but you might need these for the API layer
pub use entities::{JobDependency, MoldableDescription};

// Domain Errors
#[derive(Debug, thiserror::Error)]
pub enum JobError {
    #[error("Job {0:?} is not in a state to accept dependencies")]
    InvalidStateForDependency(JobId),

    #[error("Moldable description not found: {0}")]
    DescriptionNotFound(i32),

    #[error("Repository failure: {0}")]
    InfrastructureError(String),
}

// 5. Types used across the context
pub type Result<T> = std::result::Result<T, JobError>;
