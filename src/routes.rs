use crate::auth::{Claims, Encoder};
use crate::db::User;
use crate::graphql::schema::Schema;
use actix_web::{error::ErrorUnauthorized, post, web, HttpResponse, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GQLRequest, GQLResponse};
use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[post("/login")]
pub(crate) async fn login(
    login: web::Json<Login>,
    pool: web::Data<PgPool>,
    encoder: web::Data<Encoder>,
) -> Result<HttpResponse> {
    if User::verify_login(&login.username, &login.password, &pool)
        .await
        .map_err(|err| ErrorUnauthorized(err))?
    {
        let now = Local::now();
        let claims = Claims {
            exp: (now + Duration::from_std(encoder.expiration_time).unwrap()).timestamp(),
            sub: login.username.clone(),
        };
        let token = encoder
            .encode(&claims)
            .map_err(|err| ErrorUnauthorized(err))?;
        log::info!("{}", token);
        Ok(HttpResponse::Ok().body(token))
    } else {
        Err(ErrorUnauthorized(anyhow::anyhow!(
            "Invalid username or password provided"
        )))
    }
}

pub(crate) async fn graphql(
    schema: web::Data<Schema>,
    encoder: web::Data<Encoder>,
    req: GQLRequest,
    bearer: BearerAuth,
) -> Result<GQLResponse> {
    let token = bearer.token();
    let claims = encoder
        .decode(token)
        .map_err(|err| ErrorUnauthorized(err))?;
    Ok(req
        .into_inner()
        .data(claims.sub)
        .execute(&schema)
        .await
        .into())
}

pub(crate) async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/subscriptions"),
        )))
}
