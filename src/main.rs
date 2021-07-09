#[macro_use]
extern crate diesel;

use std::env;

use actix_web::{error, web, App, HttpRequest, HttpResponse, HttpServer};
use dotenv::dotenv;
use juniper_actix::{graphql_handler, playground_handler};

mod ddb;
mod domain;
mod graphql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or("8080".to_string());

    println!("running server on port {}", port);

    HttpServer::new(|| {
        let schema = graphql::new_schema();

        App::new()
            .data(schema)
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
    })
    .bind(format!("0.0.0.0:{}", port))
    .unwrap()
    .run()
    .await
}

async fn playground_route() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<graphql::Schema>,
) -> actix_web::Result<HttpResponse> {
    let authorized_user_id = match req.headers().get("x-user-id") {
        Some(v) => Some(v.to_str().map_err(|e| error::ErrorBadRequest(e))?.into()),
        None => None,
    };

    let context = graphql::Context { authorized_user_id };
    graphql_handler(&schema, &context, req, payload).await
}
