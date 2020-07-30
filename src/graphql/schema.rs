use crate::auth::{Claims, Encoder};
use crate::db::{search::NewSearch, user::NewUser, Search, User};
use crate::graphql::scalars::DurationString;
use async_graphql::{
    serde_json, serde_json::json, Context, ErrorExtensions, FieldError, FieldResult,
};
use chrono::{Duration, Local};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgListener, PgPool};
use std::boxed::Box;

pub(crate) struct DbUrl(pub String);

pub(crate) struct Username(pub String);

pub(crate) fn verify_token(ctx: &Context<'_>) -> FieldResult<String> {
    let token = ctx.data::<Option<String>>();
    let encoder = ctx.data::<Encoder>();
    {
        if let Some(token) = token {
            encoder.decode(&token).map(|claims| claims.sub)
        } else {
            Err(anyhow::anyhow!("No token provided"))
        }
    }
    .map_err(|err| err.extend_with(|_| json!({"code": 401})))
}

pub(crate) type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

pub(crate) fn schema(pool: PgPool, encoder: Encoder, db_url: String) -> Schema {
    async_graphql::Schema::build(Query, Mutation, Subscription)
        .data(encoder)
        .data(pool)
        .data(DbUrl(db_url))
        .finish()
}

pub(crate) struct Query;

#[async_graphql::Object]
impl Query {
    async fn get_searches(&self, ctx: &Context<'_>) -> FieldResult<Vec<Search>> {
        let pool = ctx.data::<PgPool>();
        let username = verify_token(ctx)?;
        Ok(Search::get_user_searches(&username, pool).await?)
    }
    async fn get_searches_for_subreddit(
        &self,
        ctx: &Context<'_>,
        subreddit: String,
    ) -> FieldResult<Vec<Search>> {
        let pool = ctx.data::<PgPool>();
        let username = verify_token(ctx)?;
        Ok(Search::get_for_subreddit(&username, &subreddit, pool).await?)
    }
    async fn get_user_info(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>();
        let username = verify_token(ctx)?;
        Ok(User::get_user(&username, pool).await?)
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
        Ok(User::insert(
            NewUser {
                username,
                password,
                refresh_time: refresh_time.0,
            },
            pool,
        )
        .await?)
    }
    async fn add_search(
        &self,
        ctx: &Context<'_>,
        subreddit: String,
        search_term: String,
    ) -> FieldResult<Search> {
        let pool = ctx.data::<PgPool>();
        let username = verify_token(ctx)?;
        Ok(Search::insert(
            NewSearch {
                username: username.clone(),
                search_term,
                subreddit,
            },
            pool,
        )
        .await?)
    }
    async fn delete_search(&self, ctx: &Context<'_>, id: i32) -> FieldResult<u64> {
        let pool = ctx.data::<PgPool>();
        let username = verify_token(ctx)?;
        Ok(Search::delete(id, username.clone(), pool).await?)
    }
    pub(crate) async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> FieldResult<String> {
        let pool = ctx.data::<PgPool>();
        let encoder = ctx.data::<Encoder>();
        if User::verify_login(&username, &password, &pool).await? {
            let now = Local::now();
            let claims = Claims {
                exp: (now + Duration::from_std(encoder.expiration_time).unwrap()).timestamp(),
                sub: username.clone(),
            };
            let token = encoder.encode(&claims)?;
            log::info!("{}", token);
            Ok(token)
        } else {
            Err(anyhow::anyhow!("Incorrect username or password"))?
        }
    }
}

pub(crate) struct Subscription;

#[async_graphql::Enum]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum ChangeType {
    Insert,
    Update,
    Delete,
}

#[async_graphql::SimpleObject]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SearchChange {
    pub operation: ChangeType,
    pub record: Search,
}

#[async_graphql::Subscription]
impl Subscription {
    async fn search_updates(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<SearchChange, FieldError>> {
        let url = ctx.data::<DbUrl>();
        let username = String::from(&ctx.data::<Username>().0);
        let mut listener = PgListener::new(&url.0).await.unwrap();
        listener.listen("searches_changes").await.unwrap();
        let stream = listener.into_stream();
        let update_stream = stream
            .filter_map(|result| async move { result.ok().map(|not| String::from(not.payload())) })
            .map(move |data| (data, username.clone()))
            .filter_map(|(payload, username)| async move {
                serde_json::from_str::<SearchChange>(&payload)
                    .ok()
                    .filter(|search| search.record.username == username)
                    .map(|search| Ok(search))
            });

        update_stream
    }
}
