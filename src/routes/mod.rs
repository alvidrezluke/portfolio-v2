use actix_web::{Scope, web};

mod home;
mod contact;
mod portfolio;

mod update_portfolio;

pub fn router() -> Scope {
    web::scope("")
        .service(home::index)
        .service(contact::index)
        .service(portfolio::index)
        .service(contact::contact)
        .service(update_portfolio::update_portfolio)
}