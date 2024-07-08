use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use log::{info, };

mod routes;
mod pages;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let address: String = std::env::var("ADDRESS").unwrap_or("127.0.0.1".into());
    let port: u16 = std::env::var("PORT").unwrap_or("8080".into()).parse().unwrap();

    info!("Starting server at {}:{}", address, port);

    HttpServer::new(|| {
        App::new()
            .service(routes::router())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
        .bind((address, port))?
        .run()
        .await
}
