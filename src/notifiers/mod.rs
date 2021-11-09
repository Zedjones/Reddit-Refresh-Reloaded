use crate::db::result::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

mod apprise;
mod gotify;

#[async_trait]
pub(crate) trait Notifier {
    async fn notify(&self, result: Result);
}
