use oar_domain::resource::value_objects::{ResourceNextState, ResourceState};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct ResourcePath {
    pub id: i64,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResourceStateQuery {
    pub state: Option<String>,
    pub network_address: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CreateResourceRequest {
    pub resource_type: String,
    pub network_address: String,
    pub cpuset: String,
    pub besteffort: bool,
    pub deploy: bool,
    pub desktop_computing: bool,
    pub available_upto: Option<i64>,
}

#[derive(Deserialize, JsonSchema)]
pub struct UpdateStateRequest {
    pub state: String,
}

#[derive(Serialize, JsonSchema)]
pub struct ResourceLogResponse {
    pub id: i64,
    pub attribute: String,
    pub value: String,
    pub date_start: i64,
    pub date_stop: i64,
    pub finaud_decision: bool,
}

#[derive(Serialize, JsonSchema)]
pub struct ResourceResponse {
    pub id: i64,
    pub resource_type: String,
    pub network_address: String,
    pub state: String,
    pub next_state: String,
    pub suspended_jobs: bool,
    pub scheduler_priority: i32,
    pub cpuset: String,
    pub besteffort: bool,
    pub deploy: bool,
    pub drain: bool,
    pub available_upto: i64,
    pub logs: Vec<ResourceLogResponse>,
}
