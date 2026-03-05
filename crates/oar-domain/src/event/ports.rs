use super::{aggregate::Event, value_objects::*};
use async_trait::async_trait;

pub struct NewEvent {
    pub event_type: String,
    pub job_id: i64,
    pub date: i64,
    pub description: String,
    pub hostnames: Vec<String>,
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_by_id(&self, id: &EventId) -> super::Result<Option<Event>>;
    async fn find_by_job(&self, job_id: i64) -> super::Result<Vec<Event>>;
    async fn find_by_type(&self, event_type: &str) -> super::Result<Vec<Event>>;
    async fn find_pending_check(&self) -> super::Result<Vec<Event>>; // to_check = YES
    async fn create(&self, event: NewEvent) -> super::Result<Event>;
    async fn mark_checked(&self, id: &EventId) -> super::Result<()>;
    async fn mark_checked_by_job(&self, job_id: i64) -> super::Result<()>;
}

#[async_trait]
pub trait EventService: Send + Sync {
    /// Log a new event — always sets to_check = YES initially
    async fn log(
        &self,
        event_type: &str,
        job_id: i64,
        description: String,
        hostnames: Vec<String>,
    ) -> super::Result<Event>;

    /// Mark all pending events for a job as checked after admin review
    async fn acknowledge_job_events(&self, job_id: i64) -> super::Result<()>;

    /// Fetch all events that still need admin attention
    async fn pending_review(&self) -> super::Result<Vec<Event>>;
}
