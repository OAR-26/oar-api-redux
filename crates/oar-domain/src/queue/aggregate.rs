use super::value_objects::*;

pub struct Queue {
    pub name: QueueName,
    pub priority: i32,
    pub scheduler_policy: String,
    pub state: QueueState,
}