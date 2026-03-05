use super::value_objects::*;

pub struct MoldableDescription {
    pub id: i64,
    pub walltime: i32,
    pub index: String,
    pub assigned_resource_ids: Vec<i32>,
}

pub struct JobDependency {
    pub required_id: JobId,
    pub min_start_shift: String,
    pub max_start_shift: String,
    pub index: DependencyIndex,
}

pub struct JobStateLog {
    pub id: i64,
    pub state: JobState,
    pub date_start: i64, // unix epoch
    pub date_stop: i64,  // unix epoch, 0 means still in state
}

pub struct JobType_ {
    pub id: i64,
    pub type_name: String,
    pub index: String,
}

pub struct FragJob {
    pub frag_date: i64,
    pub frag_state: FragState,
}

pub struct WalltimeChange {
    pub pending: i32,
    pub force: bool,
    pub delay_next_jobs: bool,
    pub granted: i32,
    pub granted_with_force: i32,
    pub granted_with_delay_next_jobs: i32,
}

pub struct Challenge {
    pub challenge: String,
    pub ssh_private_key: String,
    pub ssh_public_key: String,
}
