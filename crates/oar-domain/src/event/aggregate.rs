use super::{entities::*, value_objects::*};

pub struct Event {
    pub id: EventId,
    pub event_type: String,
    pub job_id: i64,
    pub date: i64, // unix epoch
    pub description: String,
    pub to_check: ToCheck, // flags events that need admin review
    pub hostnames: Vec<EventHostname>,
}
