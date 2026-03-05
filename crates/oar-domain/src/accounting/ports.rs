use super::{aggregate::AccountingWindow, value_objects::*};
use async_trait::async_trait;

pub struct AccountingQuery {
    pub window_start: Option<i64>,
    pub window_stop: Option<i64>,
    pub user: Option<String>,
    pub project: Option<String>,
    pub queue_name: Option<String>,
    pub consumption_type: Option<ConsumptionType>,
}

#[async_trait]
pub trait AccountingRepository: Send + Sync {
    async fn find(&self, query: AccountingQuery) -> super::Result<Vec<AccountingWindow>>;
    async fn find_by_user(&self, user: &str) -> super::Result<Vec<AccountingWindow>>;
    async fn find_by_project(&self, project: &str) -> super::Result<Vec<AccountingWindow>>;
    async fn find_by_window(&self, start: i64, stop: i64) -> super::Result<Vec<AccountingWindow>>;
    async fn create(&self, window: AccountingWindow) -> super::Result<AccountingWindow>;
    async fn close_window(
        &self,
        start: i64,
        user: &str,
        project: &str,
        queue: &str,
    ) -> super::Result<()>;
}

#[async_trait]
pub trait AccountingService: Send + Sync {
    /// Compute total consumption for a user across an optional time window
    async fn consumption_for_user(
        &self,
        user: &str,
        start: Option<i64>,
        stop: Option<i64>,
        consumption_type: ConsumptionType,
    ) -> super::Result<i64>;

    /// Compute total consumption for a project across an optional time window
    async fn consumption_for_project(
        &self,
        project: &str,
        start: Option<i64>,
        stop: Option<i64>,
        consumption_type: ConsumptionType,
    ) -> super::Result<i64>;

    /// Open a new accounting window when a job starts
    async fn open_window(
        &self,
        user: &str,
        project: &str,
        queue_name: &str,
        start: i64,
    ) -> super::Result<AccountingWindow>;

    /// Close and record consumption when a job ends
    async fn record_consumption(
        &self,
        user: &str,
        project: &str,
        queue_name: &str,
        start: i64,
        stop: i64,
        consumption: i64,
    ) -> super::Result<()>;
}
