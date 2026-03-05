mod aggregate;
pub mod ports;
mod value_objects;

pub use aggregate::AdmissionRule;
pub use ports::AdmissionOutcome;
pub use value_objects::{AdmissionRuleId, RuleEnabled};

#[derive(Debug, thiserror::Error)]
pub enum AdmissionError {
    #[error("Admission rule {0:?} not found")]
    NotFound(AdmissionRuleId),

    #[error("Rule expression is invalid: {0}")]
    InvalidRuleExpression(String),

    #[error("Priority {0} is already taken by another rule")]
    PriorityConflict(i32),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type Result<T> = std::result::Result<T, AdmissionError>;
