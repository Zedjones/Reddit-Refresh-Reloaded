use crate::auth::Encoder;
use crate::graphql::schema::{Schema, Username};
use actix_web::http::header::Header;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{serde_json::json, ErrorExtensions};
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenParams {
    access_token: Option<String>,
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

fn handle_ws_params(
    encoder: Encoder,
    val: async_graphql::serde_json::Value,
) -> async_graphql::FieldResult<async_graphql::Data> {
    let params: TokenParams = async_graphql::serde_json::from_value(val)?;
    let mut data = async_graphql::Data::default();
    let username = {
        if let Some(token) = params.access_token {
            encoder.decode(&token).map(|claims| claims.sub)
        } else {
            Err(anyhow::anyhow!("No token provided"))
        }
    }
    .map_err(|err| err.extend_with(|_| json!({"code": 401})))?;
    data.insert(Username(username));
    Ok(data)
}

pub(crate) async fn graphql_ws(
    encoder: web::Data<Encoder>,
    schema: web::Data<Schema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let encoder = encoder.as_ref().clone();
    ws::start_with_protocols(
        WSSubscription::new(&schema)
            .init_context_data(move |payload_val| handle_ws_params(encoder.clone(), payload_val)),
        &["graphql-ws"],
        &req,
        payload,
    )
}
