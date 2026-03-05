use super::{aggregate::GanttEntry, entities::GanttLogEntry, value_objects::*};
use async_trait::async_trait;

#[async_trait]
pub trait GanttRepository: Send + Sync {
    // Live + Visu reads
    async fn find_by_job(
        &self,
        moldable_job_id: i32,
        table: &GanttTable,
    ) -> super::Result<Option<GanttEntry>>;

    async fn find_all(&self, table: &GanttTable) -> super::Result<Vec<GanttEntry>>;

    // Live + Visu writes
    async fn upsert(&self, entry: GanttEntry, table: &GanttTable) -> super::Result<()>;

    async fn delete_by_job(&self, moldable_job_id: i32, table: &GanttTable) -> super::Result<()>;

    async fn clear(&self, table: &GanttTable) -> super::Result<()>; // full table wipe on resched

    // Log — append only, never mutated
    async fn find_log_by_sched_date(&self, sched_date: i64) -> super::Result<Vec<GanttLogEntry>>;
    async fn find_log_by_job(&self, moldable_job_id: i32) -> super::Result<Vec<GanttLogEntry>>;
    async fn append_log(&self, entry: GanttLogEntry) -> super::Result<()>;
}

#[async_trait]
pub trait GanttService: Send + Sync {
    /// Snapshot current live schedule into the log — called at each scheduling cycle
    async fn snapshot(&self, sched_date: i64) -> super::Result<()>;

    /// Promote live schedule into visu table — called when schedule is stable enough to display
    async fn publish_to_visu(&self) -> super::Result<()>;

    /// Wipe live table and rebuild from scratch — called when full reschedule is triggered
    async fn reset_live(&self) -> super::Result<()>;

    /// Fetch predicted start time for a specific job
    async fn predicted_start(&self, moldable_job_id: i32) -> super::Result<Option<i64>>;
}
