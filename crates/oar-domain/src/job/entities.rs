use super::value_objects::*;

pub struct JobDependency {
    pub required_id: JobId,
    pub index: Option<String>,
}

pub struct JobStateLog {
    pub state: JobState,
    pub date_start: chrono::NaiveDateTime,
    pub date_stop: Option<chrono::NaiveDateTime>,
}

pub struct MoldableDescription {
    pub id: i32,
    pub walltime: i32,
    pub assigned_resource_ids: Vec<i32>, // References to Resource Aggregate
}
