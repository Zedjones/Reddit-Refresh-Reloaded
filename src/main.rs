pub mod db;
mod notifiers;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv;
use env_logger::Env;
use log::error;

use db::timeout_connect;
use routes::greet;

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
            .wrap(Logger::default())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
