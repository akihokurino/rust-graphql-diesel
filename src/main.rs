mod domain;
mod graphql;

use std::env;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use juniper_actix::{graphql_handler, playground_handler};

async fn playground_route() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<graphql::Schema>,
) -> actix_web::Result<HttpResponse> {
    let context = graphql::Context {};
    graphql_handler(&schema, &context, req, payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or("8080".to_string());

    HttpServer::new(move || {
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
