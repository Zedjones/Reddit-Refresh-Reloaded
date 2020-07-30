use super::data::SearchResult;
use crate::db::{result::NewResult, Search};

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
    async fn search_reddit(&self) -> anyhow::Result<NewResult> {
        let response = self
            .client
            .get(&self.search_url)
            .send()
            .await?
            .json::<SearchResult>()
            .await?;
        log::info!("{:?}", response);
        Ok(NewResult {
            search_id: 32323,
            title: "adsf".to_string(),
        })
    }
    pub async fn check_results(&self) {
        while self.running {
            tokio::time::delay_for(self.refresh_time).await;
            self.search_reddit().await;
        }
    }
    pub fn stop(&mut self) {
        self.running = false;
    }
}
