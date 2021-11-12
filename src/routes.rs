use crate::auth::Encoder;
use crate::graphql::schema::{Schema, Username};
use actix_web::http::header::Header;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::ErrorExtensions;
use async_graphql_actix_web::{Request, Response, WSSubscription};
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
    req: Request,
    http_req: HttpRequest,
) -> Result<Response> {
    let auth = Authorization::<Bearer>::parse(&http_req).ok();
    let bearer = auth.map(|auth| auth.into_scheme().clone());
    let cow_token = bearer.map(|bearer| bearer.token().to_owned());
    let token = cow_token.map(|cow| String::from(cow));
    Ok(schema.execute(req.into_inner().data(token)).await.into())
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
    val: serde_json::Value,
) -> async_graphql::FieldResult<async_graphql::Data> {
    let params: TokenParams = serde_json::from_value(val)?;
    let mut data = async_graphql::Data::default();
    let username = {
        if let Some(token) = params.access_token {
            encoder.decode(&token).map(|claims| claims.sub)
        } else {
            Err(anyhow::anyhow!("No token provided"))
        }
    }
    .map_err(|err| err.extend_with(|_, e| e.set("code", 401)))?;
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
    WSSubscription::start_with_initializer(
        schema.as_ref().clone(),
        &req,
        payload,
        |payload_val| async move { handle_ws_params(encoder, payload_val) },
    )
}
