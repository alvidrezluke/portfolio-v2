
use actix_web::{web, get, HttpResponse, post, HttpRequest};
use actix_web::web::Bytes;
use serde::Deserialize;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use descape::UnescapeExt;

#[get("/portfolio")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body(crate::pages::get_page(crate::pages::Page::Portfolio))
}

#[derive(Deserialize)]
struct UpdateReq {
    key: String
}

type HmacSha256 = Hmac<Sha256>;

fn verify_sender(msg: &str, incoming: &str) -> Result<(), String> {
    let key = std::env::var("UPDATE_KEY").unwrap();
    let mut mac = HmacSha256::new_from_slice(key.as_ref()).unwrap();
    mac.update(msg.as_ref());
    println!("{}", hex::encode(mac.finalize().into_bytes()));
    // return match mac.verify_slice(&*hex::decode(incoming).unwrap()) {
    //     Ok(()) => Ok(()),
    //     Err(e) => Err("Invalid sender".into())
    // }
    Ok(())
}

#[post("/portfolio")]
pub async fn update_portfolio(req: HttpRequest, bytes: Bytes) -> HttpResponse {
    // Key comparison
    let incoming = req.headers().get("X-Hub-Signature-256").unwrap().to_str().unwrap().strip_prefix("sha256=").unwrap();
    println!("{}", incoming);
    let body = String::from_utf8_lossy(&bytes);
    // let mut json: serde_json::Value = serde_json::from_str(body.as_ref()).unwrap();
    // let test: serde_json::Value = serde_json::from_str(json["payload"].as_str().unwrap()).unwrap();
    // let data = json!({
    //     "payload": test
    // });
    println!("{}", &*body.to_unescaped().unwrap());
    verify_sender(&*body.to_unescaped().unwrap(), incoming).unwrap();

    crate::pages::refresh_portfolio().await;
    HttpResponse::Ok().body("Updated!")
}