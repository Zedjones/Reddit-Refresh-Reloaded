use crate::auth::{Claims, Encoder};
use crate::db::User;
use crate::graphql::schema::Schema;
use actix_identity::Identity;
use actix_web::{
    error::ErrorUnauthorized, http::header::Header, post, web, HttpRequest, HttpResponse, Result,
};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
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
    id: Identity,
    login: web::Json<Login>,
    encoder: web::Data<Encoder>,
    pool: web::Data<PgPool>,
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
        id.remember(token);
    }
    Ok(HttpResponse::Ok().into())
}

// TODO: Take in the JWT token here, and then call `.data(token)` on the builder
// to provide the token as context to the individual query
pub(crate) async fn graphql(
    schema: web::Data<Schema>,
    req: GQLRequest,
    http_req: HttpRequest,
) -> GQLResponse {
    let auth = Authorization::<Bearer>::parse(&http_req).ok();
    let bearer = auth.map(|auth| auth.into_scheme().clone());
    let cow_token = bearer.map(|bearer| bearer.token().to_owned());
    let token = cow_token.map(|cow| String::from(cow));
    req.into_inner().data(token).execute(&schema).await.into()
}

pub(crate) async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/subscriptions"),
        )))
}
