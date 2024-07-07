use actix_web::{App, HttpServer};

mod routes;
mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routes::router())).bind(("127.0.0.1", 8080))?.run().await
}
