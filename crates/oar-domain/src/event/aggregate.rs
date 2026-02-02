pub struct Event {
    pub id: i32,
    pub event_type: String,
    pub job_id: Option<i32>, // Cross-context reference
    pub description: String,
    pub hostnames: Vec<String>, // Flattened from event_log_hostnames
}