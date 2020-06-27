use chrono::{DateTime, Utc};
use sqlx::PgPool;

pub(crate) struct Result {
    id: i32,
    inserted: DateTime<Utc>,
    search_id: i32,
    title: String,
}

impl Result {
    pub async fn insert(&self, pool: PgPool) -> anyhow::Result<Self> {
        todo!()
    }
}