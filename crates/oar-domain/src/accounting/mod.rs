mod aggregate;
pub mod ports;
mod value_objects;

pub use aggregate::AccountingWindow;
pub use value_objects::ConsumptionType;

#[derive(Debug, thiserror::Error)]
pub enum AccountingError {
    #[error("Accounting window not found for user {user} starting at {window_start}")]
    WindowNotFound { user: String, window_start: i64 },

    #[error("Window for user {user} at {window_start} is already closed")]
    WindowAlreadyClosed { user: String, window_start: i64 },

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type Result<T> = std::result::Result<T, AccountingError>;
