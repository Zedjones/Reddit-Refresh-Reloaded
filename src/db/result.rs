use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;

pub(crate) struct Result {
    id: i32,
    search_id: i32,
    title: String,
    inserted: NaiveDateTime,
}

pub(crate) struct NewResult {
    search_id: i32,
    title: String,
}

impl Result {
    pub async fn insert(result: NewResult, pool: PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let result = sqlx::query!(
            "INSERT INTO results (search_id, title, inserted) \
             VALUES ($1, $2, $3) RETURNING id, search_id, title, inserted",
            result.search_id,
            result.title,
            Utc::now().naive_utc()
        )
        .fetch_one(&mut conn)
        .await?;
        conn.commit().await?;
        Ok(Result {
            id: result.id,
            search_id: result.search_id,
            title: result.title,
            inserted: result.inserted,
        })
    }
}
