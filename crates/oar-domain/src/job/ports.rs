use super::{aggregate::Job, value_objects::*};
use async_trait::async_trait;

pub struct NewJob {
    pub name: Option<String>,
    pub user: String,
    pub project: String,
    pub group: String,
    pub job_type: JobType,
    pub queue_name: String,
    pub command: Option<String>,
    pub initial_request: Option<String>,
    pub properties: Option<String>,
    pub launching_directory: String,
    pub stdout_file: Option<String>,
    pub stderr_file: Option<String>,
    pub checkpoint: i32,
    pub checkpoint_signal: i32,
    pub notify: Option<String>,
    pub env: Option<String>,
}

#[async_trait]
pub trait JobRepository: Send + Sync {
    async fn find_by_id(&self, id: &JobId) -> super::Result<Option<Job>>;
    async fn find_by_state(&self, state: &JobState) -> super::Result<Vec<Job>>;
    async fn find_by_user(&self, user: &str) -> super::Result<Vec<Job>>;
    async fn find_by_queue(&self, queue_name: &str) -> super::Result<Vec<Job>>;
    async fn create(&self, job: NewJob) -> super::Result<Job>;
    async fn update_state(&self, id: &JobId, state: JobState) -> super::Result<()>;
    async fn delete(&self, id: &JobId) -> super::Result<()>;
}

#[async_trait]
pub trait JobService: Send + Sync {
    // Lifecycle transitions — only valid transitions are exposed
    async fn submit(&self, job: NewJob) -> super::Result<Job>;
    async fn hold(&self, id: &JobId) -> super::Result<()>;
    async fn release(&self, id: &JobId) -> super::Result<()>; // Hold → Waiting
    async fn launch(&self, id: &JobId) -> super::Result<()>; // toLaunch → Launching → Running
    async fn suspend(&self, id: &JobId) -> super::Result<()>;
    async fn resume(&self, id: &JobId) -> super::Result<()>;
    async fn terminate(&self, id: &JobId, exit_code: i32) -> super::Result<()>;
    async fn mark_error(&self, id: &JobId, reason: String) -> super::Result<()>;
    async fn frag(&self, id: &JobId) -> super::Result<()>; // request kill
    async fn resubmit(&self, id: &JobId) -> super::Result<Job>;
}
