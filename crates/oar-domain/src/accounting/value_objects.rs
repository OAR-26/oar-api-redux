#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsumptionType {
    Asked, // resources requested
    Used,  // resources actually consumed
}
