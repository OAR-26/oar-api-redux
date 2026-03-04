use async_trait::async_trait;
use chrono::Utc;
use hex;
use oar_domain::iam::{
    errors::AuthError,
    models::{AuthCredential, AuthIdentity},
    ports::{ApiKeyRepository, AuthService},
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use oar_domain::iam::models::Claims;

use time::{Duration, OffsetDateTime};

pub struct AuthServiceImpl {
    pub api_key_repo: Arc<dyn ApiKeyRepository>,
    secret: String,
    expiration_seconds: i64,
}

impl AuthServiceImpl {
    pub fn new(
        secret: String,
        expiration_seconds: i64,
        api_key_repo: Arc<dyn ApiKeyRepository>,
    ) -> Self {
        Self {
            api_key_repo,
            secret,
            expiration_seconds,
        }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn authenticate(&self, credential: AuthCredential) -> Result<AuthIdentity, AuthError> {
        match credential {
            AuthCredential::BearerToken(token) => {
                let claims = self.verify_token(&token).await?;

                Ok(AuthIdentity {
                    user_id: claims.sub,
                    role: claims.role,
                })
            }

            AuthCredential::ApiKey(raw_key) => {
                let mut hasher = Sha256::new();
                hasher.update(raw_key.as_bytes());
                let hash = hex::encode(hasher.finalize());

                let key = self
                    .api_key_repo
                    .find_by_hash(&hash)
                    .await?
                    .ok_or(AuthError::InvalidApiKey)?;

                if key.expires_at.map(|e| e < Utc::now()).unwrap_or(false) {
                    return Err(AuthError::TokenExpired);
                }

                Ok(AuthIdentity {
                    user_id: key.user_id,
                    role: key.role,
                })
            }
        }
    }

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

    async fn generate_token(&self, user_id: uuid::Uuid) -> Result<String, AuthError> {
        let now = OffsetDateTime::now_utc();
        let exp = now + Duration::seconds(self.expiration_seconds);

        let claims = Claims {
            sub: user_id,
            exp: exp.unix_timestamp(),
            role: "user".to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token)
    }

    async fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;

        Ok(token_data.claims)
    }
}
