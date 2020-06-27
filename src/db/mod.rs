pub mod result;
pub mod search;
pub mod user;

use log::info;
use std::time::Duration;
use sqlx::PgPool;

const CONN_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn timeout_connect() -> Option<PgPool> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or("127.0.0.1".to_string());
    info!("Attempting to connect to Postgres at address {}", db_url);
    info!("Timeout is: {} seconds", CONN_TIMEOUT.as_secs());
    if let Ok(pool) = PgPool::builder().connect_timeout(Duration::from_secs(10)).build(&db_url).await {
        if pool.acquire().await.is_ok() {
            return Some(pool)
        }
    }
    None
}