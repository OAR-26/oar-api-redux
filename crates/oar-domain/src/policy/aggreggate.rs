pub struct Queue {
    pub name: String,
    pub priority: i32,
    pub scheduler_policy: String,
}

pub struct AdmissionRule {
    pub id: i32,
    pub priority: i32,
    pub rule_content: String,
}

pub struct AccountingRecord {
    pub window: (chrono::NaiveDateTime, chrono::NaiveDateTime),
    pub user: String,
    pub project: String,
    pub consumption: i64,
}
