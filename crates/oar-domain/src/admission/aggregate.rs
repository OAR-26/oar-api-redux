use super::value_objects::*;

pub struct AdmissionRule {
    pub id: AdmissionRuleId,
    pub priority: i32, // lower number = evaluated first
    pub enabled: RuleEnabled,
    pub rule: String, // raw rule expression evaluated by the scheduler
}
