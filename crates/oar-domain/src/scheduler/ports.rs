use super::{aggregate::Scheduler, value_objects::*};
use async_trait::async_trait;

pub struct NewScheduler {
    pub name: SchedulerName,
    pub script: String,
    pub description: String,
}

#[async_trait]
pub trait SchedulerRepository: Send + Sync {
    async fn find_by_name(&self, name: &SchedulerName) -> super::Result<Option<Scheduler>>;
    async fn find_all(&self) -> super::Result<Vec<Scheduler>>;
    async fn create(&self, scheduler: NewScheduler) -> super::Result<Scheduler>;
    async fn update_script(&self, name: &SchedulerName, script: String) -> super::Result<()>;
    async fn update_description(
        &self,
        name: &SchedulerName,
        description: String,
    ) -> super::Result<()>;
    async fn delete(&self, name: &SchedulerName) -> super::Result<()>;
}

#[async_trait]
pub trait SchedulerService: Send + Sync {
    /// Assign a scheduler to a queue — validates the scheduler exists first
    async fn assign_to_queue(
        &self,
        scheduler_name: &SchedulerName,
        queue_name: &str,
    ) -> super::Result<()>;

    /// Validate that the scheduler script path is reachable and executable
    async fn validate_script(&self, name: &SchedulerName) -> super::Result<ScriptStatus>;
}

pub enum ScriptStatus {
    Valid,
    NotFound { path: String },
    NotExecutable { path: String },
}
