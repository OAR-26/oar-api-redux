#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceId(pub i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceState {
    Alive,
    Dead,
    Suspected,
    Absent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceNextState {
    UnChanged,
    Alive,
    Dead,
    Absent,
    Suspected,
}
