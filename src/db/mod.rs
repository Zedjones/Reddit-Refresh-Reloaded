pub mod result;
pub mod search;
pub mod user;
pub(crate) use {result::Result, search::Search, user::User};

use log::info;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

const CONN_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn timeout_connect(db_url: &str) -> Option<PgPool> {
    info!("Attempting to connect to Postgres at address {}", db_url);
    info!("Timeout is: {} seconds", CONN_TIMEOUT.as_secs());
    if let Ok(pool) = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(10))
        .connect(&db_url)
        .await
    {
        if pool.acquire().await.is_ok() {
            return Some(pool);
        }
    }
    None
}
