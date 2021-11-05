use std::time::Duration;

use serde::Deserialize;

const SECONDS_IN_DAY: u64 = 86_400;

fn default_expiration() -> Duration {
    Duration::from_secs(2 * SECONDS_IN_DAY)
}

#[derive(Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) database_url: String,
    pub(crate) apprise_url: String,
    pub(crate) jwt_secret: String,
    #[serde(default = "default_expiration")]
    pub(crate) jwt_expiration: Duration,
}
