use crate::db::{Search, User};
use crate::graphql::schema::SearchChange;
use crate::scanner::Scanner;

use std::collections::HashMap;

use async_graphql::serde_json;
use futures::future::{AbortHandle, Abortable};
use sqlx::{postgres::PgListener, PgPool};
use tokio::stream::StreamExt;

pub struct Manager {
    pool: PgPool,
    search_url: String,
    scanner_map: HashMap<i32, AbortHandle>,
}

impl Manager {
    pub async fn new(pool: PgPool, search_url: String) -> anyhow::Result<Self> {
        let searches = Search::get_searches(&pool).await?;
        let mut scanner_map: HashMap<i32, AbortHandle> = HashMap::new();
        for (search, refresh_time) in searches {
            let id = search.id;
            let scanner = Scanner::new(pool.clone(), search, refresh_time);
            let (handle, registration) = AbortHandle::new_pair();
            actix_rt::spawn(async move {
                let _ = Abortable::new(scanner.check_results(), registration).await;
            });
            let handle_clone = handle.clone();
            actix_rt::spawn(async move {
                tokio::time::delay_for(std::time::Duration::from_secs(11)).await;
                log::info!("Stopping scan...");
                handle_clone.abort();
            });
            scanner_map.insert(id, handle);
        }
        Ok(Manager {
            pool,
            search_url,
            scanner_map,
        })
    }
    pub async fn handle_notification(&mut self, payload: &str) -> anyhow::Result<()> {
        let search = serde_json::from_str::<SearchChange>(payload)?;
        let id = search.record.id;
        // TODO: Differentiate between delete/insert event
        let user = User::get_user(&search.record.username, &self.pool).await?;
        let scanner = Scanner::new(self.pool.clone(), search.record, user.refresh_time);
        let (handle, registration) = AbortHandle::new_pair();
        actix_rt::spawn(async move {
            let _ = Abortable::new(scanner.check_results(), registration).await;
        });
        self.scanner_map.insert(id, handle);
        Ok(())
    }
    pub async fn monitor(mut self) -> anyhow::Result<()> {
        let mut listener = PgListener::new(&self.search_url).await?;
        listener.listen("searches_changes").await?;
        let mut stream = listener.into_stream();
        while let Some(notification) = stream.try_next().await? {
            if let Err(error) = self.handle_notification(notification.payload()).await {
                log::error!("{}", error);
            }
        }
        Ok(())
    }
}
