#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GanttEntryId(pub i64);

/// Distinguishes live scheduling data from visualization snapshots
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GanttTable {
    Live,        // gantt_jobs_predictions + gantt_jobs_resources
    Visu,        // gantt_jobs_predictions_visu + gantt_jobs_resources_visu
}