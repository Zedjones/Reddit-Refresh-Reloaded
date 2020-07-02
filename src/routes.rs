use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GQLRequest, GQLResponse};

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// FIXME: Whenever I have GQL queries working
/*
#[post("/graphql")]
async fn graphql(schema: web::Data<Schema>, req: GQLRequest) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}
*/

#[get("/graphql")]
async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/subscriptions"),
        )))
}
