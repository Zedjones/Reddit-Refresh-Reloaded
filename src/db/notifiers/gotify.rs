use sqlx::PgPool;

pub(crate) struct GotifySettings {
    enabled: bool,
    server_url: String,
    token: String,
    priority: Option<i64>,
}

impl GotifySettings {
    async fn insert(
        settings: GotifySettings,
        username: String,
        pool: &PgPool,
    ) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let gotify_settings = sqlx::query!(
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
        Ok(GotifySettings {
            enabled: gotify_settings.enabled,
            server_url: gotify_settings.server_url,
            token: gotify_settings.token,
            priority: gotify_settings.priority,
        })
    }
}
