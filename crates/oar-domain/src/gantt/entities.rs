pub struct GanttResource {
    pub resource_id: i32,
}

pub struct GanttLogEntry {
    pub sched_date: i64, // unix epoch — when this snapshot was taken
    pub moldable_job_id: i32,
    pub start_time: i64, // unix epoch
    pub resource_ids: Vec<i32>,
}
