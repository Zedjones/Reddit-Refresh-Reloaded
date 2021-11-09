use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::Notifier;
use crate::db::notifiers::apprise::AppriseConfig;

pub(crate) struct AppriseNotifier {
    base_url: String,
    config: AppriseConfig,
    client: Client,
}

#[async_trait]
impl Notifier for AppriseNotifier {
    async fn notify(&self, result: crate::db::Result) {
        let json = json!({
            "urls": self.config.uri,
            "type": self.config.urgency,
            "body": result.permalink,
            "title": result.title
        });

        let req = self
            .client
            .post(&format!("{}/notify", self.base_url))
            .json(&json);

        req.send().await.unwrap();
    }
}
