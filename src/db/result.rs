use async_graphql::SimpleObject;
use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(SimpleObject, Deserialize)]
#[graphql(complex)]
pub(crate) struct Result {
    pub id: String,
    pub search_id: i32,
    pub permalink: String,
    pub title: String,
    #[graphql(skip)]
    #[serde(with = "naive_utc")]
    pub inserted: NaiveDateTime,
    pub thumbnail: Option<String>,
}

pub(crate) struct NewResult {
    pub id: String,
    pub search_id: i32,
    pub title: String,
    pub permalink: String,
    pub thumbnail: Option<String>,
}

impl Result {
    pub async fn insert(result: NewResult, pool: &PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let result = sqlx::query!(
            "INSERT INTO results (id, search_id, title, inserted, permalink, thumbnail) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, search_id, title, inserted, permalink, thumbnail",
            result.id,
            result.search_id,
            result.title,
            Utc::now().naive_utc(),
            result.permalink,
            result.thumbnail
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
            thumbnail: result.thumbnail,
        })
    }
    pub async fn get_results_by_search(search_id: i32, pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        let results = sqlx::query!(
            "SELECT id, search_id, title, inserted, permalink, thumbnail FROM results \
             WHERE search_id = $1 \
             ORDER BY inserted DESC",
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
                thumbnail: result.thumbnail,
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
            "SELECT id, search_id, title, inserted, permalink, thumbnail FROM results \
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

mod naive_utc {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%z";

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
