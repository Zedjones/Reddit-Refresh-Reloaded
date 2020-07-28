use crate::db::Search;
use crate::scanner::Scanner;

use std::boxed::Box;
use std::collections::HashMap;
use std::time::Duration;

use futures::stream::FuturesUnordered;
use futures::Future;
use sqlx::PgPool;

pub struct Manager {
    pool: PgPool,
    scanner_map: HashMap<Search, Scanner>,
    check_results_futures: FuturesUnordered<Box<dyn Future<Output = ()>>>,
}

impl Manager {
    async fn new(pool: PgPool) -> Self {
        let test_search = Search {
            id: 100,
            search_term: "topre".to_string(),
            subreddit: "mechanicalkeyboards".to_string(),
            username: "zedjones".to_string(),
        };
        let scanner = Scanner::new(pool.clone(), test_search, Duration::from_secs(5)).await;
        Manager {
            pool,
            scanner_map: HashMap::new(),
            check_results_futures: FuturesUnordered::new(),
        }
    }
}
