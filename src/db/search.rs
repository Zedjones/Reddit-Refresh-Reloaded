use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::types::PgInterval, Done, PgPool};
use std::time::Duration;

use crate::db::User;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, SimpleObject)]
#[graphql(complex)]
pub(crate) struct Search {
    pub id: i32,
    pub username: String,
    pub subreddit: String,
    pub search_term: String,
    #[graphql(skip)]
    pub refresh_time: Option<Duration>,
}

pub(crate) struct NewSearch {
    pub username: String,
    pub subreddit: String,
    pub search_term: String,
    pub refresh_time: Option<Duration>,
}

impl Search {
    pub async fn insert(search: NewSearch, pool: &PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let search = sqlx::query!(
            "INSERT INTO searches (username, subreddit, search_term, refresh_time) \
             VALUES ($1, $2, $3, $4) RETURNING id, username, subreddit, search_term, refresh_time",
            search.username,
            search.subreddit,
            search.search_term,
            search
                .refresh_time
                .map(|time| PgInterval::try_from(time).unwrap())
        )
        .fetch_one(&mut conn)
        .await?;
        conn.commit().await?;
        Ok(Search {
            id: search.id,
            username: search.username,
            refresh_time: search
                .refresh_time
                .map(|time| User::convert_to_duration(time)),
            search_term: search.search_term,
            subreddit: search.subreddit,
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

        if deleted.rows_affected() == 0 {
            Err(anyhow::anyhow!("Invalid id or id is owned by another user"))
        } else {
            conn.commit().await?;
            Ok(deleted.rows_affected())
        }
    }
    pub async fn get_search(id: i32, pool: &PgPool) -> anyhow::Result<Self> {
        let mut conn = pool.begin().await?;
        let search = sqlx::query!(
            "SELECT id, username, subreddit, search_term, refresh_time FROM searches
             WHERE id = $1",
            id
        )
        .fetch_one(&mut conn)
        .await?;
        Ok(Search {
            id: search.id,
            username: search.username,
            refresh_time: search
                .refresh_time
                .map(|time| User::convert_to_duration(time)),
            search_term: search.search_term,
            subreddit: search.subreddit,
        })
    }
    pub async fn get_searches(pool: &PgPool) -> anyhow::Result<Vec<(Self, Duration)>> {
        let mut conn = pool.begin().await?;
        let searches: Vec<(Self, Duration)> = sqlx::query!(
            "SELECT id, searches.username, subreddit, search_term,
                users.refresh_time AS user_refresh_time,
                searches.refresh_time AS search_refresh_time
                FROM searches
                INNER JOIN users
                ON searches.username = users.username"
        )
        .fetch_all(&mut conn)
        .await?
        .into_iter()
        .map(|search| {
            (
                Search {
                    id: search.id,
                    username: search.username,
                    refresh_time: search
                        .search_refresh_time
                        .map(|time| User::convert_to_duration(time)),
                    search_term: search.search_term,
                    subreddit: search.subreddit,
                },
                User::convert_to_duration(search.user_refresh_time),
            )
        })
        .collect();
        Ok(searches)
    }
    pub async fn get_for_subreddit(
        username: &str,
        subreddit: &str,
        pool: &PgPool,
    ) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        let searches = sqlx::query!(
            "SELECT id, username, subreddit, search_term, refresh_time FROM searches \
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
                username: search.username,
                refresh_time: search
                    .refresh_time
                    .map(|time| User::convert_to_duration(time)),
                search_term: search.search_term,
                subreddit: search.subreddit,
            })
            .collect())
    }
    pub async fn get_user_searches(username: &str, pool: &PgPool) -> anyhow::Result<Vec<Self>> {
        let mut conn = pool.begin().await?;
        let searches = sqlx::query!(
            "SELECT id, username, subreddit, search_term, refresh_time FROM searches \
             WHERE username = $1",
            username
        )
        .fetch_all(&mut conn)
        .await?;
        Ok(searches
            .into_iter()
            .map(|search| Search {
                id: search.id,
                username: search.username,
                refresh_time: search
                    .refresh_time
                    .map(|time| User::convert_to_duration(time)),
                search_term: search.search_term,
                subreddit: search.subreddit,
            })
            .collect())
    }
}
