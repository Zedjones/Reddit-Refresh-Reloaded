use refinery::config::Config;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/db/migrations");
}

fn main() {
    dotenv::dotenv().ok();
    let mut config = Config::from_env_var("DATABASE_URL").unwrap();
    embedded::migrations::runner().run(&mut config).unwrap();
}