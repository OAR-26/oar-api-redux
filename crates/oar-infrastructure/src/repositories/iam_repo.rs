use async_trait::async_trait;
use chrono::{DateTime, Utc};
use oar_domain::iam::{errors::AuthError, models::ApiKey, ports::ApiKeyRepository};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresApiKeyRepository {
    pool: PgPool,
}

impl PostgresApiKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// Internal DB row — keeps sqlx mapping separate from domain model
struct ApiKeyRow {
    id: Uuid,
    user_id: Uuid,
    key_hash: String,
    name: String,
    role: String,
    expires_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl From<ApiKeyRow> for ApiKey {
    fn from(row: ApiKeyRow) -> Self {
        ApiKey {
            id: row.id,
            user_id: row.user_id,
            key_hash: row.key_hash,
            name: row.name,
            role: row.role,
            expires_at: row.expires_at,
            created_at: row.created_at,
        }
    }
}

fn hash_key(raw: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    hex::encode(hasher.finalize())
}

fn generate_raw_key() -> String {
    let mut bytes = [0u8; 32];
    let mut rng = rand::rng();
    use rand::Rng;
    rng.fill_bytes(&mut bytes);

    format!("oar_{}", hex::encode(bytes))
}

#[async_trait]
impl ApiKeyRepository for PostgresApiKeyRepository {
    async fn find_by_hash(&self, hash: &str) -> Result<Option<ApiKey>, AuthError> {
        let row = sqlx::query_as!(
            ApiKeyRow,
            r#"
            SELECT id, user_id, key_hash, name, role, expires_at, created_at
            FROM api_keys
            WHERE key_hash = $1
            "#,
            hash
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    async fn create(
        &self,
        user_id: Uuid,
        name: String,
        role: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<String, AuthError> {
        let raw_key = generate_raw_key();
        let key_hash = hash_key(&raw_key);

        sqlx::query!(
            r#"
            INSERT INTO api_keys (user_id, key_hash, name, role, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user_id,
            key_hash,
            name,
            role,
            expires_at
        )
        .execute(&self.pool)
        .await?;

        // Raw key is returned here and never stored — caller must save it
        Ok(raw_key)
    }

    async fn list_for_user(&self, user_id: Uuid) -> Result<Vec<ApiKey>, AuthError> {
        let rows = sqlx::query_as!(
            ApiKeyRow,
            r#"
            SELECT id, user_id, key_hash, name, role, expires_at, created_at
            FROM api_keys
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn revoke(&self, id: Uuid, user_id: Uuid) -> Result<(), AuthError> {
        // user_id check prevents users from revoking each other's keys
        let result = sqlx::query!(
            r#"
            DELETE FROM api_keys
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AuthError::ApiKeyNotFound);
        }

        Ok(())
    }
}
