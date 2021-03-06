use crate::db::{Result, Search, User};
use crate::graphql::scalars::{DurationString, TimestampDateTime};
use async_graphql::{Context, FieldResult};
use sqlx::PgPool;

#[async_graphql::Object]
impl Result {
    async fn id(&self) -> String {
        self.id.clone()
    }
    async fn title(&self) -> String {
        self.title.clone()
    }
    async fn inserted(&self) -> TimestampDateTime {
        self.inserted.into()
    }
    async fn url(&self) -> String {
        self.permalink.clone()
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
    async fn search_term(&self) -> String {
        self.search_term.clone()
    }
    async fn results(&self, ctx: &Context<'_>) -> FieldResult<Vec<Result>> {
        let pool = ctx.data::<PgPool>().unwrap();
        Ok(Result::get_results_by_search(self.id, pool).await?)
    }
}

#[async_graphql::Object]
impl User {
    async fn username(&self) -> String {
        self.username.clone()
    }
    async fn refresh_time(&self) -> DurationString {
        self.refresh_time.into()
    }
    async fn searches(&self, ctx: &Context<'_>) -> FieldResult<Vec<Search>> {
        let pool = ctx.data::<PgPool>().unwrap();
        Ok(Search::get_user_searches(&self.username, &pool).await?)
    }
}
