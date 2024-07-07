use actix_web::{get, HttpResponse};

#[get("/portfolio")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Portfolio!")
}