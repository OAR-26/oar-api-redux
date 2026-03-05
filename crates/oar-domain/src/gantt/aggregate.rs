use super::entities::*;

pub struct GanttEntry {
    pub moldable_job_id: i32,
    pub start_time: i64, // unix epoch — predicted start
    pub resources: Vec<GanttResource>,
}
