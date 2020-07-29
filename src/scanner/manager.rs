use crate::db::Search;
use crate::scanner::Scanner;

use std::collections::HashMap;
use std::sync::Arc;

use sqlx::PgPool;

pub struct Manager {
    pool: PgPool,
    scanner_map: HashMap<i32, Arc<Scanner>>,
}

impl Manager {
    async fn new(pool: PgPool) -> anyhow::Result<Self> {
        let searches = Search::get_searches(&pool).await?;
        let mut scanner_map: HashMap<i32, Arc<Scanner>> = HashMap::new();
        for (search, refresh_time) in searches {
            let id = search.id;
            let scanner = Arc::from(Scanner::new(pool.clone(), search, refresh_time).await);
            let scanner_copy = scanner.clone();
            actix_rt::spawn(async move { scanner_copy.clone().check_results().await });
            scanner_map.insert(id, scanner);
        }
        Ok(Manager { pool, scanner_map })
    }
}
