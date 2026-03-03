use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use oar_domain::iam::{errors::AuthError, models::Claims, ports::TokenService};

use time::{Duration, OffsetDateTime};

pub struct JwtServiceImpl {
    secret: String,
    expiration_seconds: i64,
}

impl JwtServiceImpl {
    pub fn new(secret: String, expiration_seconds: i64) -> Self {
        Self {
            secret,
            expiration_seconds,
        }
    }
}

#[async_trait]
impl TokenService for JwtServiceImpl {
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
