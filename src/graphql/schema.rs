use crate::db::{Search, User};
use async_graphql::{
    serde_json::json, Context, EmptyMutation, EmptySubscription, ErrorExtensions, FieldResult,
};
use sqlx::PgPool;

pub(crate) type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub(crate) fn schema(pool: PgPool) -> Schema {
    async_graphql::Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}

pub(crate) struct Query;

#[async_graphql::Object]
impl Query {
    async fn get_searches(&self, ctx: &Context<'_>, username: String) -> FieldResult<Vec<Search>> {
        let pool = ctx.data::<PgPool>();
        Search::get_user_searches(&username, pool)
            .await
            .map_err(|err| err.extend_with(|_| json!({"code": 500})))
    }
    async fn get_user_info(&self, ctx: &Context<'_>, username: String) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>();
        User::get_user(username, pool)
            .await
            .map_err(|err| err.extend_with(|_| json!({"code": 500})))
    }
}
