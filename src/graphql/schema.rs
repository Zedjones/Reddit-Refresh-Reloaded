use crate::db::{user::NewUser, Search, User};
use crate::graphql::scalars::DurationString;
use async_graphql::{
    serde_json::json, Context, EmptyMutation, EmptySubscription, ErrorExtensions, FieldResult,
};
use sqlx::PgPool;

pub(crate) type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub(crate) fn schema(pool: PgPool) -> Schema {
    async_graphql::Schema::build(Query, Mutation, EmptySubscription)
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

pub(crate) struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
        refresh_time: DurationString,
    ) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>();
        User::insert(
            NewUser {
                username,
                password,
                refresh_time: refresh_time.0,
            },
            pool,
        )
        .await
        .map_err(|err| err.extend_with(|_| json!({"code": 500})))
    }
}