use super::{entities::*, value_objects::*};

pub struct Resource {
    pub id: ResourceId,
    pub network_address: String,
    pub state: ResourceState,
    pub cpuset: i32,
    // Aggregated Entities
    pub logs: Vec<ResourceLog>,
}