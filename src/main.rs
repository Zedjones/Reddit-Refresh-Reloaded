mod auth;
pub mod db;
mod graphql;
mod notifiers;
mod routes;
mod scanner;

use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};
use dotenv;
use env_logger::Env;
use log::error;
use serde::Deserialize;
use std::time::Duration;

use auth::Encoder;
use db::timeout_connect;
use graphql::schema::schema;
use routes::{graphql as graphql_handler, graphql_playground, graphql_ws};

const SECONDS_IN_DAY: u64 = 86_400;

fn default_expiration() -> Duration {
    Duration::from_secs(2 * SECONDS_IN_DAY)
}

#[derive(Deserialize)]
struct Config {
    database_url: String,
    jwt_secret: String,
    #[serde(default = "default_expiration")]
    jwt_expiration: Duration,
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let config = envy::from_env::<Config>().unwrap_or_else(|err| {
        error!("Could not load a required value from the environment");
        error!("Error: {}", err);
        std::process::exit(1);
    });

    let pool = timeout_connect(&config.database_url)
        .await
        .unwrap_or_else(|| {
            error!("Could not connect to the database.");
            std::process::exit(1);
        });

    let encoder = Encoder {
        expiration_time: config.jwt_expiration,
        secret: config.jwt_secret.clone(),
    };

    let db_url = config.database_url.clone();

    let test_search = db::Search {
        id: 100,
        search_term: "topre".to_string(),
        subreddit: "mechanicalkeyboards".to_string(),
        username: "zedjones".to_string(),
    };
    let scanner = scanner::Scanner::new(pool.clone(), test_search, Duration::from_secs(5)).await;
    let results_future = scanner.check_results();

    let server_future = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema(pool.clone(), encoder.clone(), db_url.clone()))
            .data(encoder.clone())
            .data(db_url.clone())
            .wrap(Logger::default())
            .service(
                web::resource("/subscriptions")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(graphql_ws),
            )
            .service(graphql_handler)
            .service(graphql_playground)
    })
    .bind("127.0.0.1:8000")?
    .run();

    let (server_res, _) = tokio::join!(server_future, results_future);
    server_res?;
    Ok(())
}
