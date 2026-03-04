#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    // --- Token errors ---
    #[error("Invalid token format")]
    InvalidToken,

    #[error("Token has expired")]
    TokenExpired,

    // --- Credential errors ---
    #[error("Wrong credentials")]
    WrongCredentials,

    #[error("Missing credentials")]
    MissingCredentials,

    #[error("Password hashing failed")]
    PasswordHashingFailed,

    #[error("Password verification failed")]
    PasswordVerificationFailed,

    // --- API Key errors ---
    #[error("API key not found")]
    ApiKeyNotFound,

    #[error("API key revoked")]
    ApiKeyRevoked,

    #[error("API key already exists")]
    ApiKeyAlreadyExists,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Invalid Scheme")]
    InvalidScheme,
    // --- Generic infrastructure failure ---
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
