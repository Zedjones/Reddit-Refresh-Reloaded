mod db;
mod notifiers;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv;
use env_logger::Env;
use log::info;
use sqlx::postgres::PgPool;
use std::time::Duration;

use routes::greet;

const CONN_TIMEOUT: Duration = Duration::from_secs(10);

async fn timeout_connect() -> Option<PgPool> {
    let db_url = std::env::var("POSTGRES_URL").unwrap_or("127.0.0.1".to_string());
    let start = std::time::Instant::now();
    info!("Attempting to connect to Postgres at address {}", db_url);
    info!("Timeout is: {} seconds", CONN_TIMEOUT.as_secs());
    if let Ok(pool) = PgPool::builder().build(&db_url).await {
        loop {
            if pool.try_acquire().is_some() {
                return Some(pool);
            } else if start.elapsed() > CONN_TIMEOUT {
                return None;
            }
            info!("Sleeping...");
            info!("Elapsed: {}", start.elapsed().as_secs());
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    None
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let pool = timeout_connect().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
