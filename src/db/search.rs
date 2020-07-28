use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Search {
    pub id: i32,
    pub username: String,
    pub subreddit: String,
    pub search_term: String,
}

pub(crate) struct NewSearch {
    pub username: String,
    pub subreddit: String,
    pub search_term: String,
}

impl Search {
    pub async fn insert(search: NewSearch, pool: &PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let search = sqlx::query!(
            "INSERT INTO searches (username, subreddit, search_term) \
             VALUES ($1, $2, $3) RETURNING id, username, subreddit, search_term",
            search.username,
            search.subreddit,
            search.search_term
        )
        .fetch_one(&mut conn)
        .await?;
        conn.commit().await?;
        Ok(Search {
            id: search.id,
            username: search.username,
            subreddit: search.subreddit,
            search_term: search.search_term,
        })
    }
    pub async fn delete(id: i32, username: String, pool: &PgPool) -> anyhow::Result<u64> {
        let mut conn = pool.begin().await?;
        let _results_deleted = sqlx::query!("DELETE FROM results WHERE search_id = $1", id,)
            .execute(&mut conn)
            .await?;
        let deleted = sqlx::query!(
            "DELETE FROM searches WHERE id = $1 AND username = $2",
            id,
            username
        )
        .execute(&mut conn)
        .await?;

        if deleted == 0 {
            Err(anyhow::anyhow!("Invalid id or id is owned by another user"))
        } else {
            conn.commit().await?;
            Ok(deleted)
        }
    }
    pub async fn get_search(id: i32, pool: PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let search = sqlx::query!(
            "SELECT id, username, subreddit, search_term FROM searches
             WHERE id = $1",
            id
        )
        .fetch_one(&mut conn)
        .await?;
        Ok(Search {
            id: search.id,
            username: search.username,
            subreddit: search.subreddit,
            search_term: search.search_term,
        })
    }
    pub async fn get_for_subreddit(
        username: &str,
        subreddit: &str,
        pool: &PgPool,
    ) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        let searches = sqlx::query!(
            "SELECT id, username, subreddit, search_term FROM searches \
             WHERE username = $1 AND subreddit = $2",
            username,
            subreddit
        )
        .fetch_all(&mut conn)
        .await?;
        Ok(searches
            .into_iter()
            .map(|search| Search {
                id: search.id,
                search_term: search.search_term,
                subreddit: search.subreddit,
                username: search.username,
            })
            .collect())
    }
    pub async fn get_user_searches(username: &str, pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        let searches = sqlx::query!(
            "SELECT id, username, subreddit, search_term FROM searches \
             WHERE username = $1",
            username
        )
        .fetch_all(&mut conn)
        .await?;
        Ok(searches
            .into_iter()
            .map(|search| Search {
                id: search.id,
                search_term: search.search_term,
                subreddit: search.subreddit,
                username: search.username,
            })
            .collect())
    }
}
