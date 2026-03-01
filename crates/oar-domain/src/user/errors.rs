use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token format")]
    InvalidToken,
    #[error("Token has expired")]
    TokenExpired,
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Password hashing failed")]
    PasswordHashingFailed,
    #[error("Password verification failed")]
    PasswordVerificationFailed,
}