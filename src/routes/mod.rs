use actix_web::{Scope, web};

mod home;
mod contact;
mod portfolio;

pub fn router() -> Scope {
    web::scope("")
        .service(home::index)
        .service(contact::index)
        .service(portfolio::index)
        .service(contact::contact)
        .service(portfolio::update_portfolio)
}