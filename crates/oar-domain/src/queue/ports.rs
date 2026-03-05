use super::{aggregate::Queue, value_objects::*};
use async_trait::async_trait;

pub struct NewQueue {
    pub name: QueueName,
    pub priority: i32,
    pub scheduler_policy: String,
}

#[async_trait]
pub trait QueueRepository: Send + Sync {
    async fn find_by_name(&self, name: &QueueName) -> super::Result<Option<Queue>>;
    async fn find_all(&self) -> super::Result<Vec<Queue>>;
    async fn find_active(&self) -> super::Result<Vec<Queue>>;
    async fn create(&self, queue: NewQueue) -> super::Result<Queue>;
    async fn update_priority(&self, name: &QueueName, priority: i32) -> super::Result<()>;
    async fn delete(&self, name: &QueueName) -> super::Result<()>;
}

#[async_trait]
pub trait QueueService: Send + Sync {
    async fn activate(&self, name: &QueueName) -> super::Result<()>;
    async fn deactivate(&self, name: &QueueName) -> super::Result<()>;
    // Changing scheduler policy requires validation — not all policies are valid
    async fn change_policy(&self, name: &QueueName, policy: String) -> super::Result<()>;
}
