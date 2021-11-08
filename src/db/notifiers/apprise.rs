use sqlx::PgPool;

/// Apprise Gerneric Configuration
pub(crate) struct AppriseConfig {
    id: i32,
    name: String,
    uri: String,
}

impl AppriseConfig {
    pub async fn get_configs_for_user(username: &str, pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        Ok(sqlx::query_as!(
            AppriseConfig,
            "SELECT id, name, uri FROM notifier_configs WHERE username = $1",
            username,
        )
        .fetch_all(&mut conn)
        .await?)
    }
}
