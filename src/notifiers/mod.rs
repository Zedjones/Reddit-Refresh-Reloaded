use crate::db::result::Result;
use async_trait::async_trait;
mod gotify;

#[async_trait]
pub(crate) trait Notifier {
    async fn notify(&self, result: Result);
}
