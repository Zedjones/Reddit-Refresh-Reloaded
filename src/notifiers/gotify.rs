// Notifier to write to a file
use async_trait::async_trait;
use reqwest::Client;

use super::Notifier;

pub(crate) struct GotifyNotifier {
    server_url: String,
    token: String,
    priority: Option<i64>,
    client: Client,
}

#[async_trait]
impl Notifier for GotifyNotifier {
    async fn notify(&self, result: crate::db::Result) -> anyhow::Result<()> {
        let mut req = self
            .client
            .post(&format!("{}/message?token={}", self.server_url, self.token))
            .header("title", result.title)
            .header("message", result.permalink);
        if let Some(priority) = self.priority {
            req = req.header("priority", priority);
        }
        Ok(req.send().await.map(|_| ())?)
    }
}
