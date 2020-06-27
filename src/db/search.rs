use sqlx::{postgres::PgRow, PgPool, Row};

pub(crate) struct Search {
    pub id: i32,
    pub username: String,
    pub subreddit: String,
    pub search_term: String,
}

const insert_str: &str = "INSERT INTO searches (username, subreddit, search_term) \
                          VALUES ($1, $2, $3) RETURNING id, username, subreddit, search_term";

pub(crate) struct NewSearch {
    username: String,
    subreddit: String,
    search_term: String,
}

impl Search {
    pub async fn insert(search: NewSearch, pool: PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let search = sqlx::query(insert_str)
            .bind(search.username)
            .bind(search.subreddit)
            .bind(search.search_term)
            .map(|row: PgRow| Search {
                id: row.get(0),
                username: row.get(1),
                subreddit: row.get(2),
                search_term: row.get(3),
            })
            .fetch_one(&mut conn)
            .await?;
        conn.commit().await?;
        Ok(search)
    }
}
