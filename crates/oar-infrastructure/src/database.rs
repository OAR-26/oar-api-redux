use sqlx::{PgPool, postgres::PgConnectOptions, ConnectOptions};
use std::time::Duration;
use tracing::info;
use tracing::log::LevelFilter;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let connect_options = database_url.parse::<PgConnectOptions>()?
        .log_statements(LevelFilter::Debug)
        .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1));

    let pool = PgPool::connect_with(connect_options).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    info!("Database connection pool created successfully");
    Ok(pool)
}

pub async fn close_pool(pool: PgPool) {
    info!("Closing database connection pool");
    pool.close().await;
}
