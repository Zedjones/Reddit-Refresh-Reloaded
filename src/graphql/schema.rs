use crate::auth::{Claims, Encoder};
use crate::db::{search::NewSearch, user::NewUser, Search, User};
use crate::graphql::scalars::DurationString;
use async_graphql::{serde_json::json, Context, EmptySubscription, ErrorExtensions, FieldResult};
use chrono::{Duration, Local};
use sqlx::PgPool;

async fn verify_token(ctx: &Context<'_>) -> FieldResult<String> {
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

pub(crate) type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

pub(crate) fn schema(pool: PgPool, encoder: Encoder) -> Schema {
    async_graphql::Schema::build(Query, Mutation, EmptySubscription)
        .data(encoder)
        .data(pool)
        .finish()
}

pub(crate) struct Query;

#[async_graphql::Object]
impl Query {
    async fn get_searches(&self, ctx: &Context<'_>) -> FieldResult<Vec<Search>> {
        let pool = ctx.data::<PgPool>();
        let username = verify_token(ctx).await?;
        Ok(Search::get_user_searches(&username, pool).await?)
    }
    async fn get_searches_for_subreddit(
        &self,
        ctx: &Context<'_>,
        subreddit: String,
    ) -> FieldResult<Vec<Search>> {
        let pool = ctx.data::<PgPool>();
        let username = verify_token(ctx).await?;
        Ok(Search::get_for_subreddit(&username, &subreddit, pool).await?)
    }
    async fn get_user_info(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>();
        let username = verify_token(ctx).await?;
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
        let username = verify_token(ctx).await?;
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
        let username = verify_token(ctx).await?;
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
