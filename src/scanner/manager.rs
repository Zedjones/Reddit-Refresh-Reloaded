use crate::db::Search;
use crate::scanner::Scanner;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::{AbortHandle, Abortable};
use sqlx::PgPool;

pub struct Manager {
    pool: PgPool,
    scanner_map: HashMap<i32, AbortHandle>,
}

impl Manager {
    pub async fn new(pool: PgPool) -> anyhow::Result<Self> {
        let searches = Search::get_searches(&pool).await?;
        let mut scanner_map: HashMap<i32, AbortHandle> = HashMap::new();
        for (search, refresh_time) in searches {
            let id = search.id;
            let scanner = Scanner::new(pool.clone(), search, refresh_time).await;
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
        Ok(Manager { pool, scanner_map })
    }
}
