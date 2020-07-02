use crate::db::{user::NewUser, Result, Search, User};
use async_graphql::{serde_json::json, Context, ErrorExtensions, FieldResult};
use sqlx::PgPool;
use std::time::Duration;

#[async_graphql::Object]
impl Result {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn title(&self) -> String {
        self.title.clone()
    }
    async fn inserted(&self) -> String {
        self.inserted.to_string()
    }
}

#[async_graphql::Object]
impl Search {
    async fn id(&self) -> i32 {
        self.id
    }
    async fn username(&self) -> String {
        self.username.clone()
    }
    async fn subreddit(&self) -> String {
        self.subreddit.clone()
    }
    async fn results(&self, ctx: &Context<'_>) -> FieldResult<Vec<Result>> {
        let pool = ctx.data::<PgPool>();
        // Get results, return a FieldError if there is an error
        Result::get_results_by_search(self.id, pool)
            .await
            .map_err(|err| err.extend_with(|_| json!({"code": 500})))
    }
}

#[async_graphql::Object]
impl User {
    async fn username(&self) -> String {
        self.username.clone()
    }
    async fn refresh_time(&self) -> u64 {
        self.refresh_time.as_secs()
    }
}
