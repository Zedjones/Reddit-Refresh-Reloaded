use crate::db::{result::NewResult, Search, User};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use std::time::Duration;

#[derive(Deserialize, Debug)]
struct ChildResult {
    title: String,
    permalink: String,
}

#[derive(Deserialize, Debug)]
struct Child {
    data: ChildResult,
}

#[derive(Deserialize, Debug)]
struct Children {
    children: Vec<Child>,
}

#[derive(Deserialize, Debug)]
struct SearchResult {
    data: Children,
}

pub(crate) struct Scanner {
    pool: PgPool,
    search: Search,
    search_url: String,
    running: bool,
    refresh_time: Duration,
    client: Client,
}

impl Scanner {
    pub async fn new(pool: PgPool, search: Search) -> Self {
        let user = User::get_user(&search.username, &pool).await.unwrap();
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
            refresh_time: user.refresh_time,
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
        println!("{:?}", response);
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
}
