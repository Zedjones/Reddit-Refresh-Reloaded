use crate::graphql::schema::Schema;
use actix_web::http::header::Header;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[post("/graphql")]
pub(crate) async fn graphql(
    schema: web::Data<Schema>,
    req: GQLRequest,
    http_req: HttpRequest,
) -> Result<GQLResponse> {
    let auth = Authorization::<Bearer>::parse(&http_req).ok();
    let bearer = auth.map(|auth| auth.into_scheme().clone());
    let cow_token = bearer.map(|bearer| bearer.token().to_owned());
    let token = cow_token.map(|cow| String::from(cow));
    Ok(req.into_inner().data(token).execute(&schema).await.into())
}

#[get("/graphql")]
pub(crate) async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/subscriptions"),
        )))
}

pub(crate) async fn graphql_ws(
    schema: web::Data<Schema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}
