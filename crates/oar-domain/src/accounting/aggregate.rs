use super::value_objects::*;

pub struct AccountingWindow {
    pub window_start: i64, // unix epoch
    pub window_stop: i64,  // unix epoch, 0 = still open
    pub user: String,
    pub project: String,
    pub queue_name: String,
    pub consumption_type: ConsumptionType,
    pub consumption: i64, // in core-seconds
}
