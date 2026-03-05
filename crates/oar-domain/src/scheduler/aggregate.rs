use super::value_objects::*;

pub struct Scheduler {
    pub name: SchedulerName,
    pub script: String, // path to the scheduler script OAR will invoke
    pub description: String,
}
