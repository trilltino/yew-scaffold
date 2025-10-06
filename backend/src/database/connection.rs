use sqlx::PgPool;
use tracing::{info, error};

pub type DbPool = PgPool;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    info!("Attempting database connection...");
    info!("   Database URL: {}...", &database_url[..50]);

    match PgPool::connect(database_url).await {
        Ok(pool) => {
            info!("Database pool created successfully");

            // Test the connection
            match sqlx::query("SELECT 1").fetch_one(&pool).await {
                Ok(_) => {
                    info!("Database connection test successful");
                    Ok(pool)
                }
                Err(e) => {
                    error!("Database connection test failed: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            error!("Failed to create database pool: {}", e);
            Err(e)
        }
    }
}