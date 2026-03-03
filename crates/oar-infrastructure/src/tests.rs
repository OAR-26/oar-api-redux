#[cfg(test)]
mod tests {

    use crate::database::create_pool;
    use crate::repositories::iam_repo::PostgresApiKeyRepository;
    use crate::repositories::user_repo::PostgresUserRepository;
    use chrono::{Duration, SubsecRound, Utc};
    use oar_domain::iam::errors::AuthError;
    use oar_domain::iam::models::ApiKey;
    use oar_domain::iam::ports::ApiKeyRepository;
    use oar_domain::user::models::User;
    use oar_domain::user::ports::UserRepository;
    use sha2::{Digest, Sha256};
    use uuid::Uuid;

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    async fn setup_test_db() -> PostgresUserRepository {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!("DATABASE_URL is not set.");
        });

        let pool = create_pool(&db_url).await.unwrap_or_else(|e| {
            panic!("Failed to connect to the test database. Error: {e}");
        });

        PostgresUserRepository::new(pool)
    }

    async fn setup_test_db_api_key() -> PostgresApiKeyRepository {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!("DATABASE_URL is not set.");
        });

        let pool = create_pool(&db_url).await.unwrap_or_else(|e| {
            panic!("Failed to connect to the test database. Error: {e}");
        });

        PostgresApiKeyRepository::new(pool)
    }

    fn unique_email() -> String {
        format!("test+{}@example.com", Uuid::new_v4())
    }

    fn unique_username() -> String {
        format!("testuser_{}", &Uuid::new_v4().to_string()[..8])
    }

    /// Helper to create a valid user in the DB to satisfy Foreign Key constraints
    async fn create_dummy_user(user_repo: &PostgresUserRepository) -> Uuid {
        let original = User {
            id: Uuid::new_v4(),
            email: unique_email(),
            username: unique_username(),
            password_hash: "hashed_password".to_string(),
        };

        let created = user_repo
            .create_user(original)
            .await
            .expect("Helper failed to create dummy user");

        created.id
    }

    // -------------------------------------------------------------------------
    // Unit tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_unique_email_format() {
        let email = unique_email();
        assert!(email.contains('@'));
        assert!(email.starts_with("test+"));
        assert!(email.ends_with("@example.com"));
    }

    #[test]
    fn test_unique_emails_differ() {
        assert_ne!(unique_email(), unique_email());
    }

    // -------------------------------------------------------------------------
    // User Integration tests
    // -------------------------------------------------------------------------

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_find_by_email_not_found() {
        let repo = setup_test_db().await;
        let result = repo
            .find_by_email("nonexistent@example.com")
            .await
            .expect("query failed");
        assert!(result.is_none());
    }

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_find_by_id_not_found() {
        let repo = setup_test_db().await;
        let random_id = Uuid::new_v4();
        let result = repo.find_by_id(random_id).await.expect("query failed");
        assert!(result.is_none());
    }

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_create_and_find_user() {
        let repo = setup_test_db().await;
        let original = User {
            id: Uuid::new_v4(),
            email: unique_email(),
            username: unique_username(),
            password_hash: "hashed_password".to_string(),
        };

        let created = repo
            .create_user(original.clone())
            .await
            .expect("create_user failed");
        assert_eq!(created.email, original.email);

        let by_email = repo.find_by_email(&original.email).await.unwrap().unwrap();
        assert_eq!(by_email.id, created.id);
    }

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_duplicate_email_rejected() {
        let repo = setup_test_db().await;
        let email = unique_email();

        let first = User {
            id: Uuid::new_v4(),
            email: email.clone(),
            username: unique_username(),
            password_hash: "hash".to_string(),
        };
        let second = User {
            id: Uuid::new_v4(),
            email: email.clone(),
            username: unique_username(),
            password_hash: "hash".to_string(),
        };

        repo.create_user(first).await.unwrap();
        let duplicate_result = repo.create_user(second).await;
        assert!(duplicate_result.is_err());
    }

    // -------------------------------------------------------------------------
    // API Key Integration tests
    // -------------------------------------------------------------------------

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_find_by_hash_not_found() {
        let repo = setup_test_db_api_key().await;
        let result = repo
            .find_by_hash("nonexistent_hash")
            .await
            .expect("query failed");
        assert!(result.is_none());
    }

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_create_and_find_api_key() {
        let user_repo = setup_test_db().await;
        let repo = setup_test_db_api_key().await;

        // 1. Create a real user first to satisfy foreign key
        let user_id = create_dummy_user(&user_repo).await;

        let name = "test_service".to_string();
        let role = "admin".to_string();

        let raw_key = repo
            .create(user_id, name.clone(), role.clone(), None)
            .await
            .expect("create API key failed");
        let key_hash = format!("{:x}", sha2::Sha256::digest(raw_key.as_bytes()));

        let found_key = repo.find_by_hash(&key_hash).await.unwrap().unwrap();

        assert_eq!(found_key.user_id, user_id);
        assert_eq!(found_key.name, name);
    }

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_list_for_user() {
        let user_repo = setup_test_db().await;
        let repo = setup_test_db_api_key().await;

        // 1. Create valid users
        let user_id = create_dummy_user(&user_repo).await;
        let other_user_id = create_dummy_user(&user_repo).await;

        repo.create(user_id, "service1".to_string(), "admin".to_string(), None)
            .await
            .unwrap();
        repo.create(user_id, "service2".to_string(), "user".to_string(), None)
            .await
            .unwrap();
        repo.create(
            other_user_id,
            "other_service".to_string(),
            "admin".to_string(),
            None,
        )
        .await
        .unwrap();

        let user_keys = repo
            .list_for_user(user_id)
            .await
            .expect("list_for_user failed");

        assert_eq!(user_keys.len(), 2);
        for key in &user_keys {
            assert_eq!(key.user_id, user_id);
        }
    }

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_revoke_api_key() {
        let user_repo = setup_test_db().await;
        let repo = setup_test_db_api_key().await;

        let user_id = create_dummy_user(&user_repo).await;
        let name = "test_service".to_string();

        let raw_key = repo
            .create(user_id, name, "admin".to_string(), None)
            .await
            .unwrap();
        let key_hash = format!("{:x}", sha2::Sha256::digest(raw_key.as_bytes()));

        let found_key = repo.find_by_hash(&key_hash).await.unwrap().unwrap();

        repo.revoke(found_key.id, user_id)
            .await
            .expect("revoke API key failed");

        let result = repo.find_by_hash(&key_hash).await.unwrap();
        assert!(result.is_none(), "Expected API key to be revoked");
    }

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_revoke_api_key_wrong_user() {
        let user_repo = setup_test_db().await;
        let repo = setup_test_db_api_key().await;

        let user_id = create_dummy_user(&user_repo).await;
        let other_user_id = create_dummy_user(&user_repo).await;

        let raw_key = repo
            .create(
                user_id,
                "test_service".to_string(),
                "admin".to_string(),
                None,
            )
            .await
            .unwrap();
        let key_hash = format!("{:x}", sha2::Sha256::digest(raw_key.as_bytes()));

        let found_key = repo.find_by_hash(&key_hash).await.unwrap().unwrap();

        // Try to revoke with wrong user_id
        let revoke_result = repo.revoke(found_key.id, other_user_id).await;
        assert!(revoke_result.is_err());

        // Verify the key still exists
        let result = repo.find_by_hash(&key_hash).await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL"]
    async fn test_create_api_key_with_expiration() {
        let user_repo = setup_test_db().await;
        let repo = setup_test_db_api_key().await;

        let user_id = create_dummy_user(&user_repo).await;
        let name = "test_service".to_string();
        let expires_at = Some((Utc::now() + Duration::hours(1)).trunc_subsecs(6));
        let raw_key = repo
            .create(user_id, name.clone(), "admin".to_string(), expires_at)
            .await
            .unwrap();
        let key_hash = format!("{:x}", sha2::Sha256::digest(raw_key.as_bytes()));

        let found_key = repo.find_by_hash(&key_hash).await.unwrap().unwrap();
        assert_eq!(found_key.expires_at, expires_at);
    }
}
