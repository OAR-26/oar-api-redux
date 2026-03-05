use super::{entities::*, value_objects::*};

pub struct Resource {
    pub id: ResourceId,
    pub resource_type: String,
    pub network_address: String,
    pub state: ResourceState,
    pub next_state: ResourceNextState,
    pub finaud_decision: bool,
    pub next_finaud_decision: bool,
    pub state_num: i32,
    pub suspended_jobs: bool,
    pub scheduler_priority: i32,
    pub cpuset: String,
    pub besteffort: bool,
    pub deploy: bool,
    pub expiry_date: i64, // unix epoch
    pub desktop_computing: bool,
    pub last_job_date: i64,  // unix epoch
    pub available_upto: i64, // unix epoch, default 2147483647 = indefinitely available
    pub last_available_upto: i64,
    pub drain: bool,
    // Aggregated entities
    pub logs: Vec<ResourceLog>,
}
