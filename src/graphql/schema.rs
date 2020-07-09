use crate::auth::Encoder;
use crate::db::{user::NewUser, Search, User};
use crate::graphql::scalars::DurationString;
use async_graphql::{Context, EmptySubscription, FieldResult};
use sqlx::PgPool;

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
    async fn get_searches(&self, ctx: &Context<'_>, username: String) -> FieldResult<Vec<Search>> {
        let pool = ctx.data::<PgPool>();
        Ok(Search::get_user_searches(&username, pool).await?)
    }
    async fn get_user_info(&self, ctx: &Context<'_>, username: String) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>();
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
    /*
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
            Err(FieldError::from(anyhow::anyhow!(
                "Incorrect username or password"
            )))
        }
    }
    */
}
