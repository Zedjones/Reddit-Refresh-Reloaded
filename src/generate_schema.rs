mod auth;
mod config;
mod db;
mod graphql;

use graphql::schema::{Query, Mutation, Subscription, Schema};
use env_logger::Env;
use dotenv;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let schema: Schema = async_graphql::Schema::build(Query, Mutation, Subscription).finish();
    std::fs::write("frontend/schema.graphql", &schema.sdl()).expect("Couldn't write schema file");
}
