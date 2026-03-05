#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueState {
    Active,
    NotActive,
}
