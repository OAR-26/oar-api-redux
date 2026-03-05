#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobId(pub i64); // bigserial → i64

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobState {
    Waiting,
    Hold,
    ToLaunch,
    ToError,
    ToAckReservation,
    Launching,
    Running,
    Suspended,
    Resuming,
    Finishing,
    Terminated,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobType {
    Interactive,
    Passive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReservationState {
    None,
    ToSchedule,
    Scheduled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyIndex {
    Current,
    Log,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FragState {
    Leon,
    TimerArmed,
    LeonExterminate,
    Fragged,
}
