use sqlx::{postgres::PgRow, PgPool, Row};

pub(crate) struct Search {
    pub id: i32,
    pub username: String,
    pub subreddit: String,
    pub search_term: String,
}

pub(crate) struct NewSearch {
    username: String,
    subreddit: String,
    search_term: String,
}

impl Search {
    pub async fn insert(search: NewSearch, pool: PgPool) -> anyhow::Result<Self> {
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
}
