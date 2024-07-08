use actix_web::{get, HttpResponse, post};

#[get("/contact")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body(crate::pages::get_page(crate::pages::Page::Contact))
}

#[post("/contact")]
pub async fn contact() -> HttpResponse {
    HttpResponse::Ok().body("Recieved contact")
}