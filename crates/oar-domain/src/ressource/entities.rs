use super::value_objects::*;

pub struct ResourceLog {
    pub id: i64,
    pub attribute: String,
    pub value: String,
    pub date_start: i64, // unix epoch
    pub date_stop: i64,  // unix epoch, 0 = still active
    pub finaud_decision: bool,
}
