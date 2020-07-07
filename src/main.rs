mod auth;
pub mod db;
mod graphql;
mod notifiers;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv;
use env_logger::Env;
use log::{error, info};
use std::time::Duration;

use auth::Encoder;
use db::timeout_connect;
use graphql::schema::schema;
use routes::{graphql as graphql_handler, graphql_playground};

const SECONDS_IN_DAY: u32 = 86_400;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let pool = timeout_connect().await.unwrap_or_else(|| {
        error!("Could not connect to the database.");
        std::process::exit(1);
    });

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        error!("No JWT secret provided");
        std::process::exit(1);
    });

    let expiration_time: Duration = match std::env::var("JWT_EXPIRATION").ok() {
        Some(expiration) => Duration::from_secs(expiration.parse::<u64>().unwrap_or_else(|_| {
            error!("JWT timeout was not a number");
            std::process::exit(1);
        })),
        None => {
            info!("No JWT expiration provided, using default of 2 days");
            std::time::Duration::from_secs((2 * SECONDS_IN_DAY) as u64)
        }
    };

    let encoder = Encoder {
        expiration_time,
        secret: jwt_secret,
    };

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema(pool.clone(), encoder.clone()))
            .wrap(Logger::default())
            .service(graphql_handler)
            .service(graphql_playground)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
