use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;

pub(crate) struct Result {
    pub id: i32,
    pub search_id: i32,
    pub title: String,
    pub inserted: NaiveDateTime,
}

pub(crate) struct NewResult {
    pub search_id: i32,
    pub title: String,
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
    pub async fn get_results_by_search(search_id: i32, pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        let results = sqlx::query!(
            "SELECT id, search_id, title, inserted FROM results \
             WHERE search_id = $1",
            search_id
        )
        .fetch_all(&mut conn)
        .await?;
        Ok(results
            .into_iter()
            .map(|result| Result {
                id: result.id,
                inserted: result.inserted,
                search_id: result.search_id,
                title: result.title,
            })
            .collect())
    }
}
