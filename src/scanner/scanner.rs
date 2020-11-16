use super::data::SearchResult;
use crate::db::{result::NewResult, Result as DbResult, Search};

use reqwest::Client;
use sqlx::PgPool;

use std::time::Duration;

pub(crate) struct Scanner {
    pool: PgPool,
    search: Search,
    search_url: String,
    running: bool,
    refresh_time: Duration,
    client: Client,
}

impl Scanner {
    pub fn new(pool: PgPool, search: Search, refresh_time: Duration) -> Self {
        let client = Client::new();
        let search_url = format!(
            "https://old.reddit.com/r/{}/search.json?q={}&sort=new&restrict_sr=on",
            &search.subreddit, &search.search_term
        )
        .to_string();
        Scanner {
            pool,
            search,
            search_url,
            client,
            running: true,
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
        }))
    }
    pub async fn check_results(&self) {
        while self.running {
            tokio::time::delay_for(self.refresh_time).await;
            let search_result = self.search_reddit().await;
            let res = match search_result {
                Err(error) => Err(error),
                Ok(Some(new_result)) => {
                    match DbResult::get_latest_result_for_search(self.search.id, &self.pool).await {
                        Err(error) => Err(error),
                        Ok(Some(old_result)) => {
                            if old_result.id != new_result.id {
                                DbResult::insert(new_result, &self.pool).await.map(|_| ())
                            } else {
                                Ok(())
                            }
                        }
                        Ok(None) => DbResult::insert(new_result, &self.pool).await.map(|_| ()),
                    }
                }
                Ok(None) => Ok(()),
            };
            if let Err(error) = res {
                log::error!("{}", error);
            }
        }
    }
    pub fn stop(&mut self) {
        self.running = false;
    }
}
