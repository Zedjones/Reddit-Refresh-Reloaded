mod auth;
mod config;
mod db;
mod graphql;
mod notifiers;
mod routes;
mod scanner;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};
use dotenv;
use env_logger::Env;
use log::error;

use auth::Encoder;
use config::Config;
use db::timeout_connect;
use graphql::schema::schema;
use routes::{graphql as graphql_handler, graphql_playground, graphql_ws, index};
use scanner::manager::Manager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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
    let manager = Manager::new(pool.clone(), db_url.clone()).await?;
    let monitor_handle = tokio::spawn(async move {
        if let Err(error) = manager.monitor().await {
            log::error!("{}", error);
        }
    });

    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);

    let server_handle = HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(pool.clone())
            .data(schema(pool.clone(), encoder.clone(), db_url.clone()))
            .data(encoder.clone())
            .data(db_url.clone())
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(
                web::resource("/subscriptions")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(graphql_ws),
            )
            .service(graphql_handler)
            .service(graphql_playground)
            // Serve SPA
            .service(fs::Files::new("/", *config.clone().frontend_dir).index_file("index.html"))
            .default_service(web::resource("").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    let (monitor_res, sys_res) = tokio::join!(monitor_handle, sys);
    match (monitor_res, sys_res) {
        (Err(e), _) => Err(anyhow::anyhow!(e)),
        (_, Err(e)) => Err(anyhow::anyhow!(e)),
        _ => Ok(server_handle),
    }
}
