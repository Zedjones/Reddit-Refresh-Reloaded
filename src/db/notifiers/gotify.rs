use sqlx::PgPool;

#[derive(Clone)]
pub(crate) struct GotifySettings {
    pub enabled: bool,
    pub server_url: String,
    pub token: String,
    pub priority: Option<i64>,
}

impl GotifySettings {
    async fn insert(
        settings: GotifySettings,
        username: String,
        pool: &PgPool,
    ) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let gotify_settings = sqlx::query_as!(
            GotifySettings,
            "INSERT INTO gotify_settings (username, enabled, server_url, token, priority) \
             VALUES ($1, $2, $3, $4, $5) RETURNING enabled, server_url, token, priority",
            username,
            settings.enabled,
            settings.server_url,
            settings.token,
            settings.priority
        )
        .fetch_one(&mut conn)
        .await?;
        conn.commit().await?;
        Ok(gotify_settings)
    }
    pub async fn get_settings_for_user(username: &str, pool: &PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let gotify_settings = sqlx::query_as!(
            GotifySettings,
            "SELECT enabled, server_url, token, priority FROM gotify_settings \
             WHERE username = $1",
            username
        )
        .fetch_one(&mut conn)
        .await?;
        Ok(gotify_settings)
    }
}
