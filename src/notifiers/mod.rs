use crate::db::result::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

mod apprise;
mod gotify;

#[derive(Serialize, Deserialize)]
pub(crate) enum Urgency {
    Info,
    Success,
    Warning,
    Failure,
}

impl Default for Urgency {
    fn default() -> Self {
        Urgency::Info
    }
}

#[async_trait]
pub(crate) trait Notifier {
    async fn notify(&self, result: Result);
}
