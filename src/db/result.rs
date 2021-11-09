use async_graphql::SimpleObject;
use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;

#[derive(SimpleObject)]
#[graphql(complex)]
pub(crate) struct Result {
    pub id: String,
    pub search_id: i32,
    pub permalink: String,
    pub title: String,
    #[graphql(skip)]
    pub inserted: NaiveDateTime,
}

pub(crate) struct NewResult {
    pub id: String,
    pub search_id: i32,
    pub title: String,
    pub permalink: String,
}

impl Result {
    pub async fn insert(result: NewResult, pool: &PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let result = sqlx::query!(
            "INSERT INTO results (id, search_id, title, inserted, permalink) \
             VALUES ($1, $2, $3, $4, $5) RETURNING id, search_id, title, inserted, permalink",
            result.id,
            result.search_id,
            result.title,
            Utc::now().naive_utc(),
            result.permalink,
        )
        .fetch_one(&mut conn)
        .await?;
        conn.commit().await?;
        Ok(Result {
            id: result.id,
            search_id: result.search_id,
            title: result.title,
            inserted: result.inserted,
            permalink: result.permalink,
        })
    }
    pub async fn get_results_by_search(search_id: i32, pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        let results = sqlx::query!(
            "SELECT id, search_id, title, inserted, permalink FROM results \
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
                permalink: result.permalink,
            })
            .collect())
    }
    pub async fn get_latest_result_for_search(
        search_id: i32,
        pool: &PgPool,
    ) -> anyhow::Result<Option<Self>> {
        let mut conn = pool.begin().await?;
        let result = sqlx::query_as!(
            Result,
            "SELECT id, search_id, title, inserted, permalink FROM results \
             WHERE search_id = $1 \
             ORDER BY inserted DESC \
             LIMIT 1",
            search_id
        )
        .fetch_all(&mut conn)
        .await?;
        Ok(result.into_iter().next())
    }
}
