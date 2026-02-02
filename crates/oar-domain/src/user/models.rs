//  basic to get things started
#[derive(Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    // place holder
    pub password_hash: String,
}
