use async_trait::async_trait;
use oar_domain::user::{models::User, ports::UserRepository};
use sqlx::{FromRow, PgPool, query_as};
use uuid::Uuid;

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    username: String,
    password_hash: String,
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, user: User) -> Result<User, String> {
        let result = query_as!(
            UserRow,
            r#"
            INSERT INTO users (id, email, username, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, username, password_hash
            "#,
            user.id,
            user.email,
            user.username,
            user.password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

        Ok(User {
            id: result.id,
            email: result.email,
            username: result.username,
            password_hash: result.password_hash,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        let result = query_as!(
            UserRow,
            r#"
            SELECT id, email, username, password_hash
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find user by ID: {}", e))?;

        match result {
            Some(row) => Ok(Some(User {
                id: row.id,
                email: row.email,
                username: row.username,
                password_hash: row.password_hash,
            })),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let result = query_as!(
            UserRow,
            r#"
            SELECT id, email, username, password_hash
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to find user by email: {}", e))?;

        match result {
            Some(row) => Ok(Some(User {
                id: row.id,
                email: row.email,
                username: row.username,
                password_hash: row.password_hash,
            })),
            None => Ok(None),
        }
    }
}
