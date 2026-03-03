#[cfg(test)]
mod tests {

    use crate::database::create_pool;
    use crate::repositories::user_repo::PostgresUserRepository;
    use oar_domain::user::models::User;
    use oar_domain::user::ports::UserRepository;
    use uuid::Uuid;

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    /// Connects to the test database.
    ///
    /// # Panics
    /// - If `DATABASE_URL` is not set (tells you exactly what to do)
    /// - If the pool cannot be created (tells you the connection error)
    ///
    /// Never silently returns — every failure is loud.
    async fn setup_test_db() -> PostgresUserRepository {

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            panic!(
                "\n\n\
                DATABASE_URL is not set.\n\
                These are integration tests that require a running Postgres instance.\n\n\
                To run them:\n\
                  DATABASE_URL=postgres://user:pass@localhost/testdb cargo test -- --ignored\n\n\
                To run only unit tests (no database needed):\n\
                  cargo test --lib\n"
            )
        });

        let pool = create_pool(&db_url).await.unwrap_or_else(|e| {
            panic!(
                "\n\n\
                Failed to connect to the test database.\n\
                DATABASE_URL = {db_url}\n\
                Error: {e}\n\n\
                Make sure Postgres is running and the URL is correct.\n"
            )
        });

        PostgresUserRepository::new(pool)
    }

    /// Generates a unique email address to prevent collisions between test runs.
    fn unique_email() -> String {
        format!("test+{}@example.com", Uuid::new_v4())
    }

    /// Generates a unique username to prevent collisions between test runs.
    fn unique_username() -> String {
        format!("testuser_{}", &Uuid::new_v4().to_string()[..8])
    }

    // -------------------------------------------------------------------------
    // Unit tests — no database, always run
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
    // Integration tests — require DATABASE_URL, opt-in via `--ignored`
    // -------------------------------------------------------------------------

    /// Verifies that querying a non-existent email returns None.
    ///
    /// Run with: DATABASE_URL=... cargo test -- --ignored
    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL (run with -- --ignored)"]
    async fn test_find_by_email_not_found() {
        let repo = setup_test_db().await;

        let result = repo
            .find_by_email("nonexistent@example.com")
            .await
            .expect("find_by_email query failed");

        assert!(
            result.is_none(),
            "Expected no user for a nonexistent email, but got: {result:?}"
        );
    }

    /// Verifies that querying a non-existent UUID returns None.
    ///
    /// Run with: DATABASE_URL=... cargo test -- --ignored
    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL (run with -- --ignored)"]
    async fn test_find_by_id_not_found() {
        let repo = setup_test_db().await;
        let random_id = Uuid::new_v4();

        let result = repo
            .find_by_id(random_id)
            .await
            .expect("find_by_id query failed");

        assert!(
            result.is_none(),
            "Expected no user for id {random_id}, but got: {result:?}"
        );
    }

    /// Full lifecycle: create a user then retrieve them by email and by ID.
    ///
    /// Run with: DATABASE_URL=... cargo test -- --ignored
    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL (run with -- --ignored)"]
    async fn test_create_and_find_user() {
        let repo = setup_test_db().await;

        let original = User {
            id: Uuid::new_v4(),
            email: unique_email(),
            username: unique_username(),
            password_hash: "hashed_password".to_string(),
        };

        // --- create ---
        let created = repo
            .create_user(original.clone())
            .await
            .expect("create_user failed");

        assert_eq!(created.email, original.email, "email mismatch after create");
        assert_eq!(
            created.username, original.username,
            "username mismatch after create"
        );

        // --- find by email ---
        let by_email = repo
            .find_by_email(&original.email)
            .await
            .expect("find_by_email failed")
            .unwrap_or_else(|| {
                panic!(
                    "Expected to find user by email '{}' but got None",
                    original.email
                )
            });

        assert_eq!(by_email.email, original.email);
        assert_eq!(by_email.id, created.id);

        // --- find by id ---
        let by_id = repo
            .find_by_id(created.id)
            .await
            .expect("find_by_id failed")
            .unwrap_or_else(|| panic!("Expected to find user by id '{}' but got None", created.id));

        assert_eq!(by_id.id, created.id);
        assert_eq!(by_id.email, original.email);
    }

    /// Verifies that creating two users with the same email is rejected.
    ///
    /// Run with: DATABASE_URL=... cargo test -- --ignored
    #[tokio::test]
    #[ignore = "integration test: requires DATABASE_URL (run with -- --ignored)"]
    async fn test_duplicate_email_rejected() {
        let repo = setup_test_db().await;
        let email = unique_email();

        let first = User {
            id: Uuid::new_v4(),
            email: email.clone(),
            username: unique_username(),
            password_hash: "hash_one".to_string(),
        };

        let second = User {
            id: Uuid::new_v4(),
            email: email.clone(),
            username: unique_username(),
            password_hash: "hash_two".to_string(),
        };

        repo.create_user(first)
            .await
            .expect("first create_user failed unexpectedly");

        let duplicate_result = repo.create_user(second).await;

        assert!(
            duplicate_result.is_err(),
            "Expected an error when inserting a duplicate email, but create_user succeeded"
        );
    }
}
