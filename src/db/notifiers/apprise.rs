use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Type};

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Copy, Enum, Type)]
#[sqlx(rename_all = "lowercase")]
pub(crate) enum Urgency {
    Info,
    Success,
    Warning,
    Failure,
}

impl Default for Urgency {
    fn default() -> Self {
        Urgency::Info
    }
}

/// Apprise Generic Configuration
#[derive(SimpleObject)]
pub(crate) struct AppriseConfig {
    pub id: i32,
    /// Name of this configuration
    pub name: String,
    /// Apprise URI associated with this configuration
    pub uri: String,
    /// Priority/urgency with which to send messages
    pub urgency: Urgency,
}

impl AppriseConfig {
    pub async fn get_configs_for_user(username: &str, pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        Ok(sqlx::query_as!(
            AppriseConfig,
            r#"SELECT id, name, uri, urgency as "urgency: _" FROM notifier_configs WHERE username = $1"#,
            username,
        )
        .fetch_all(&mut conn)
        .await?)
    }
    pub async fn insert(
        config: AppriseConfig,
        username: &str,
        pool: &PgPool,
    ) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let new_config = sqlx::query_as!(
            AppriseConfig,
            r#"INSERT INTO notifier_configs (username, name, uri, urgency)
         VALUES ($1, $2, $3, $4) RETURNING id, name, uri, urgency as "urgency: _""#,
            username,
            config.name,
            config.uri,
            config.urgency as _
        )
        .fetch_one(&mut conn)
        .await?;
        conn.commit().await?;
        Ok(new_config)
    }
}
