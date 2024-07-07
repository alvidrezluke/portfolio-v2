use actix_web::{get, HttpResponse, post};

#[get("/contact")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Contact!")
}

#[post("/contact")]
pub async fn contact() -> HttpResponse {
    HttpResponse::Ok().body("Recieved contact")
}