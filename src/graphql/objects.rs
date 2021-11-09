use crate::db::{notifiers::apprise::AppriseConfig, Result, Search, User};
use crate::graphql::scalars::{DurationString, TimestampDateTime};
use async_graphql::{Context, FieldResult};
use sqlx::PgPool;

#[async_graphql::ComplexObject]
impl Result {
    async fn inserted(&self) -> TimestampDateTime {
        self.inserted.into()
    }
}

#[async_graphql::ComplexObject]
impl Search {
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
    async fn settings(&self, ctx: &Context<'_>) -> FieldResult<Vec<AppriseConfig>> {
        let pool = ctx.data::<PgPool>().unwrap();
        Ok(AppriseConfig::get_configs_for_user(&self.username, &pool).await?)
    }
}
