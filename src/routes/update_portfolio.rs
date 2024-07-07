use actix_web::{post, HttpResponse};

#[post("/update-portfolio")]
pub async fn update_portfolio() -> HttpResponse {
    // rehydrate portfolio
    crate::pages::rehydrate().await;
    HttpResponse::Ok().body("Rehydrating!")
}