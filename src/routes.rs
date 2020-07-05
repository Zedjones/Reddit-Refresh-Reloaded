use crate::graphql::schema::Schema;
use actix_web::{get, post, web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GQLRequest, GQLResponse};

// TODO: Take in the JWT token here, and then call `.data(token)` on the builder
// to provide the token as context to the individual query
#[post("/graphql")]
pub(crate) async fn graphql(schema: web::Data<Schema>, req: GQLRequest) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}

#[get("/graphql")]
pub(crate) async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/subscriptions"),
        )))
}
