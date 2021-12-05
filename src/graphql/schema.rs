use crate::auth::{Claims, Encoder};
use crate::db::notifiers::apprise::{AppriseConfig, Urgency};
use crate::db::{search::NewSearch, Result as DbResult, Search, User};
use crate::graphql::scalars::DurationString;
use async_graphql::{Context, Enum, ErrorExtensions, FieldError, FieldResult, SimpleObject};
use chrono::{Duration, Local};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgListener, PgPool};
use std::boxed::Box;

pub(crate) struct DbUrl(pub String);

pub(crate) struct Username(pub String);

pub(crate) fn verify_token(ctx: &Context<'_>) -> FieldResult<String> {
    let token = ctx.data::<Option<String>>().unwrap();
    let encoder = ctx.data::<Encoder>().unwrap();
    {
        if let Some(token) = token {
            encoder.decode(&token).map(|claims| claims.sub)
        } else {
            Err(anyhow::anyhow!("No authorization token provided"))
        }
    }
    .map_err(|err| err.extend_with(|_, e| e.set("code", 401)))
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
        let pool = ctx.data::<PgPool>().unwrap();
        let username = verify_token(ctx)?;
        Ok(Search::get_user_searches(&username, pool).await?)
    }
    async fn get_searches_for_subreddit(
        &self,
        ctx: &Context<'_>,
        subreddit: String,
    ) -> FieldResult<Vec<Search>> {
        let pool = ctx.data::<PgPool>().unwrap();
        let username = verify_token(ctx)?;
        Ok(Search::get_for_subreddit(&username, &subreddit, pool).await?)
    }
    async fn get_user_info(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let username = verify_token(ctx)?;
        Ok(User::get_user(&username, pool).await?)
    }
}

pub(crate) struct Mutation;

#[async_graphql::Object]
impl Mutation {
    /// Create a user with the provided username, password, and refresh time
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
        refresh_time: DurationString,
    ) -> FieldResult<String> {
        let pool = ctx.data::<PgPool>().unwrap();
        User::insert(
            User {
                username: username.clone(),
                password: password.clone(),
                refresh_time: refresh_time.0,
                notifiers: Vec::new(),
            },
            pool,
        )
        .await?;
        Ok(self.login(ctx, username, password).await?)
    }
    async fn add_search(
        &self,
        ctx: &Context<'_>,
        subreddit: String,
        search_term: String,
        refresh_time: Option<DurationString>,
    ) -> FieldResult<Search> {
        let pool = ctx.data::<PgPool>().unwrap();
        let username = verify_token(ctx)?;
        Ok(Search::insert(
            NewSearch {
                username: username.clone(),
                search_term,
                subreddit,
                refresh_time: refresh_time.map(|time| time.0),
            },
            pool,
        )
        .await?)
    }
    async fn delete_search(&self, ctx: &Context<'_>, id: i32) -> FieldResult<u64> {
        let pool = ctx.data::<PgPool>().unwrap();
        let username = verify_token(ctx)?;
        Ok(Search::delete(id, username.clone(), pool).await?)
    }
    pub(crate) async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> FieldResult<String> {
        let pool = ctx.data::<PgPool>().unwrap();
        let encoder = ctx.data::<Encoder>().unwrap();
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
    pub(crate) async fn add_notifier(
        &self,
        ctx: &Context<'_>,
        name: String,
        uri: String,
        priority: Urgency,
    ) -> FieldResult<AppriseConfig> {
        let pool = ctx.data::<PgPool>().unwrap();
        let username = verify_token(ctx)?;
        Ok(AppriseConfig::insert(
            AppriseConfig {
                id: 0,
                name,
                uri,
                urgency: priority,
            },
            &username,
            &pool,
        )
        .await?)
    }
}

pub(crate) struct Subscription;

#[derive(Serialize, Copy, Clone, Eq, PartialEq, Deserialize, Debug, Enum)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum ChangeType {
    Insert,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize, Debug, SimpleObject)]
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
        let url = ctx.data::<DbUrl>().unwrap();
        let username = String::from(&ctx.data::<Username>().unwrap().0);
        let mut listener = PgListener::connect(&url.0).await.unwrap();
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
    async fn result_updates(
        &self,
        ctx: &Context<'_>,
        search_id: i32,
    ) -> impl Stream<Item = Result<DbResult, FieldError>> {
        // These are fine to be unwrapped since we know that they'll be present in the context
        let url = ctx.data::<DbUrl>().unwrap();
        let username = &ctx.data::<Username>().unwrap().0;
        let pool = ctx.data::<PgPool>().unwrap();

        // Use an IIFE to take advantage of the ? operator
        let stream_result = (|| async {
            // See if there is a search associated with the provided ID
            let search = Search::get_search(search_id, &pool).await?;
            // Check if the located search is associated with the current user
            if search.username != *username {
                Err(anyhow::anyhow!(
                    "Trying to get results for a search that belongs to another user."
                ))
            } else {
                let mut listener = PgListener::connect(&url.0).await?;
                listener.listen("result_changes").await?;
                Ok(listener.into_stream())
            }
        })()
        .await;

        match stream_result {
            Err(error) => futures::stream::iter(vec![Err(error.into())].into_iter()).boxed(),
            Ok(stream) => stream
                .filter_map(
                    |result| async move { result.ok().map(|not| String::from(not.payload())) },
                )
                .filter_map(move |payload| async move {
                    serde_json::from_str::<DbResult>(&payload)
                        .ok()
                        .filter(|result| result.search_id == search_id)
                        .map(|result| Ok(result))
                })
                .boxed(),
        }
    }
}
