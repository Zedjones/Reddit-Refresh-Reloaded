use super::data::SearchResult;
use crate::db::{result::NewResult, Result as DbResult, Search};

use reqwest::Client;
use sqlx::PgPool;

use std::time::Duration;

pub(crate) struct Scanner {
    pool: PgPool,
    search: Search,
    search_url: String,
    refresh_time: Duration,
    client: Client,
}

impl Scanner {
    pub fn new(pool: PgPool, search: Search, refresh_time: Duration) -> Self {
        let client = Client::new();
        let search_url = format!(
            "https://old.reddit.com/r/{}/search.json?q={}&sort=new&restrict_sr=on&limit=1",
            &search.subreddit, &search.search_term
        )
        .to_string();
        log::trace!(
            "Creating scanner for {}: {}",
            search.subreddit,
            search.search_term
        );
        Scanner {
            pool,
            search,
            search_url,
            client,
            refresh_time,
        }
    }
    async fn search_reddit(&self) -> anyhow::Result<Option<NewResult>> {
        let response = self
            .client
            .get(&self.search_url)
            .send()
            .await?
            .json::<SearchResult>()
            .await?;
        log::info!("{:?}", response);
        Ok(response.get_latest_result().map(|result| NewResult {
            id: result.id,
            title: result.title,
            search_id: self.search.id,
            permalink: result.url,
            timestamp: result.created_utc,
            thumbnail: if result.thumbnail == "" {
                None
            } else {
                Some(result.thumbnail)
            },
        }))
    }
    pub async fn check_results(&self) {
        loop {
            log::trace!(
                "Checking results for {}: {}",
                self.search.subreddit,
                self.search.search_term
            );
            let search_result = self.search_reddit().await;
            let res = match search_result {
                Err(error) => Err(error),
                Ok(Some(new_result)) => {
                    match DbResult::get_result_for_search(
                        self.search.id,
                        &new_result.id,
                        &self.pool,
                    )
                    .await
                    {
                        Err(error) => Err(error),
                        Ok(Some(old_result)) => DbResult::update_timestamp(
                            self.search.id,
                            &old_result.id,
                            new_result.timestamp,
                            &self.pool,
                        )
                        .await
                        .map(|_| ()),
                        Ok(None) => DbResult::insert(new_result, &self.pool).await.map(|_| ()),
                    }
                }
                Ok(None) => Ok(()),
            };
            if let Err(error) = res {
                log::error!("{}", error);
            }
            // Use the search refresh time, or the user refresh time if there is none
            let wait_time = self.search.refresh_time.unwrap_or(self.refresh_time);
            tokio::time::delay_for(wait_time).await;
        }
    }
}
