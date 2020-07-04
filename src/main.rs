pub mod db;
mod graphql;
mod notifiers;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv;
use env_logger::Env;
use log::error;

use db::timeout_connect;
use graphql::schema::schema;
use routes::{graphql as graphql_handler, graphql_playground};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let pool = timeout_connect().await.unwrap_or_else(|| {
        error!("Could not connect to the database.");
        std::process::exit(1);
    });

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(schema(pool.clone()))
            .wrap(Logger::default())
            .service(graphql_handler)
            .service(graphql_playground)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
