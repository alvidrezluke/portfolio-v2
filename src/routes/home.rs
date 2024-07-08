use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body(crate::pages::get_page(crate::pages::Page::Home))
}
