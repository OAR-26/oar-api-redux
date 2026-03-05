#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventId(pub i64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToCheck {
    Yes,
    No,
}
