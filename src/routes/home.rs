use actix_web::{get, HttpResponse};
use tera::Tera;

#[get("/")]
pub async fn index() -> HttpResponse {
    let mut tera =0;
    HttpResponse::Ok().body("Hello world!")
}
