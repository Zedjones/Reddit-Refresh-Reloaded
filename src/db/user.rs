use sqlx::PgPool;
use std::time::Duration;

pub(crate) struct User {
    token: Option<String>,
    email: String,
    password: String,
    refresh_time: Duration,
}

impl User {
    pub async fn insert(&self, pool: PgPool) -> anyhow::Result<Self> {
        todo!()
    }
}
