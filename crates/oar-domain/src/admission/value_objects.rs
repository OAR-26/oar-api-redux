#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionRuleId(pub i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleEnabled {
    Yes,
    No,
}