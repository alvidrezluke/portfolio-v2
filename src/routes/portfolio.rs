use actix_web::{web, get, HttpResponse, post};
use serde::Deserialize;

#[get("/portfolio")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body(crate::pages::get_page(crate::pages::Page::Portfolio))
}

#[derive(Deserialize)]
struct UpdateReq {
    key: String
}

#[post("/portfolio")]
pub async fn update_portfolio(k: web::Json<UpdateReq>) -> HttpResponse {
    // Key comparison
    let key = std::env::var("UPDATE_KEY");
    if key.is_err() { return HttpResponse::Ok().body("Private Key could not be found!") }
    println!("{}", key.unwrap());
    // if !key.unwrap().eq(&k.key) { return HttpResponse::Ok().body("Invalid Private Key!") }

    crate::pages::refresh_portfolio().await;
    HttpResponse::Ok().body("Updated!")
}