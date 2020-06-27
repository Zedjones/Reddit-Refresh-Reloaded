use refinery::config::Config;
use log::error;
use env_logger::Env;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/db/migrations");
}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    dotenv::dotenv().ok();
    let mut config = Config::from_env_var("DATABASE_URL").unwrap();
    match embedded::migrations::runner().run(&mut config) {
        Err(_) => {
            error!("Could not run migrations, is the database online and info correct?");
            std::process::exit(1);
        },
        _ => ()
    };
}