use super::{aggregate::AdmissionRule, value_objects::*};
use async_trait::async_trait;

pub struct NewAdmissionRule {
    pub priority: i32,
    pub rule: String,
}

#[async_trait]
pub trait AdmissionRuleRepository: Send + Sync {
    async fn find_by_id(&self, id: &AdmissionRuleId) -> super::Result<Option<AdmissionRule>>;
    async fn find_all(&self) -> super::Result<Vec<AdmissionRule>>;
    async fn find_enabled(&self) -> super::Result<Vec<AdmissionRule>>; // ordered by priority
    async fn create(&self, rule: NewAdmissionRule) -> super::Result<AdmissionRule>;
    async fn update_rule(&self, id: &AdmissionRuleId, rule: String) -> super::Result<()>;
    async fn update_priority(&self, id: &AdmissionRuleId, priority: i32) -> super::Result<()>;
    async fn delete(&self, id: &AdmissionRuleId) -> super::Result<()>;
}

#[async_trait]
pub trait AdmissionService: Send + Sync {
    /// Enable a rule — idempotent if already enabled
    async fn enable(&self, id: &AdmissionRuleId) -> super::Result<()>;

    /// Disable a rule without deleting it
    async fn disable(&self, id: &AdmissionRuleId) -> super::Result<()>;

    /// Evaluate all enabled rules against a job in priority order.
    /// Returns the first rule that rejects the job, or Ok if all pass.
    async fn evaluate(&self, job_id: i64) -> super::Result<AdmissionOutcome>;
}

pub enum AdmissionOutcome {
    Accepted,
    Rejected {
        rule_id: AdmissionRuleId,
        reason: String,
    },
}
