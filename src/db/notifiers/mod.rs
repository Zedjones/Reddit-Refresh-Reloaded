use gotify::GotifySettings;
use sqlx::PgPool;

pub mod apprise;
pub mod gotify;

pub(crate) struct NotifierSettings {
    pub gotify_settings: Option<gotify::GotifySettings>,
}

impl NotifierSettings {
    pub fn new() -> Self {
        NotifierSettings {
            gotify_settings: None,
        }
    }
    pub async fn get_settings_for_user(username: &str, pool: &PgPool) -> Self {
        let gotify_settings = GotifySettings::get_settings_for_user(username, &pool)
            .await
            .ok();
        Self { gotify_settings }
    }
}
