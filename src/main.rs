mod auth;
pub mod db;
mod graphql;
mod notifiers;
mod routes;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv;
use env_logger::Env;
use log::error;
use serde::Deserialize;
use std::time::Duration;

use crate::auth::validate_token;
use auth::Encoder;
use db::timeout_connect;
use graphql::schema::schema;
use routes::{graphql as graphql_handler, graphql_playground, login};

const SECONDS_IN_DAY: u64 = 86_400;

fn default_domain() -> String {
    "localhost:8000".to_string()
}

fn default_expiration() -> Duration {
    Duration::from_secs(2 * SECONDS_IN_DAY)
}

#[derive(Deserialize)]
struct Config {
    database_url: String,
    jwt_secret: String,
    #[serde(default = "default_expiration")]
    jwt_expiration: Duration,
    #[serde(default = "default_domain")]
    domain: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
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

    let cookie_secret_key = config.jwt_secret.clone();
    let domain = config.domain.clone();
    let auth_age = chrono::Duration::from_std(config.jwt_expiration).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema(pool.clone(), encoder.clone()))
            .data(encoder.clone())
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(&domain)
                    .max_age_time(auth_age)
                    .secure(false),
            ))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_handler))
                    .route(web::get().to(graphql_playground))
                    .wrap(HttpAuthentication::bearer(validate_token)),
            )
            .service(login)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
