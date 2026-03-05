mod aggregate;
mod entities;
pub mod ports;
mod value_objects;

pub use aggregate::GanttEntry;
pub use entities::{GanttLogEntry, GanttResource};
pub use value_objects::{GanttEntryId, GanttTable};

#[derive(Debug, thiserror::Error)]
pub enum GanttError {
    #[error("Gantt entry for moldable job {0} not found")]
    NotFound(i32),

    #[error("Cannot snapshot — live schedule is empty")]
    EmptySchedule,

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type Result<T> = std::result::Result<T, GanttError>;
