#[derive(Debug)]
pub struct JobId(pub i32);
pub enum JobState {
    Waiting,
    Running,
    Terminated,
    Error,
    Suspended,
}
pub struct JobType(pub String);
