use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use oar_domain::user::errors::AuthError;
use oar_domain::user::ports::PasswordService;

pub struct Argon2PasswordService;

#[async_trait]
impl PasswordService for Argon2PasswordService {
    async fn hash_password(&self, plain: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(plain.as_bytes(), &salt)
            .map_err(|_| AuthError::PasswordHashingFailed)?
            .to_string();
        Ok(password_hash)
    }

    async fn verify_password(&self, plain: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash =
            argon2::PasswordHash::new(hash).map_err(|_| AuthError::PasswordVerificationFailed)?;
        let argon2 = Argon2::default();
        let result = argon2.verify_password(plain.as_bytes(), &parsed_hash);
        match result {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
