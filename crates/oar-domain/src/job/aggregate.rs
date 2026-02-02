use super::{entities::*, value_objects::*};
use chrono::NaiveDateTime;

pub struct Job {
    pub id: JobId,
    pub name: Option<String>,
    pub user: String,
    pub project: String,
    pub state: JobState,
    pub submission_time: NaiveDateTime,
    // Aggregated Entities
    pub descriptions: Vec<MoldableDescription>,
    pub dependencies: Vec<JobDependency>,
    pub state_logs: Vec<JobStateLog>,
}
