pub struct ResourceId(pub i32);
pub enum ResourceState {
    Alive,
    Dead,
    Absent,
    Suspected,
}
