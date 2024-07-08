mod portfolio;

use std::sync::Mutex;
use lazy_static::lazy_static;
use tera::{Context, Tera};
use log::{error};

lazy_static! {
    static ref TEMPLATES: Tera = {
        return match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

lazy_static! {
    static ref PAGES: Mutex<Pages> = Mutex::new(Pages {
        home: TEMPLATES.render("index.html", &Context::new()).unwrap(),
        portfolio: TEMPLATES.render("portfolio.html", &portfolio::get_context()).unwrap(),
        contact: TEMPLATES.render("contact.html", &Context::new()).unwrap(),
    });
}

struct Pages {
    home: String,
    portfolio: String,
    contact: String
}

pub enum Page {
    Home,
    Portfolio,
    Contact
}

pub fn get_page(page: Page) -> String {
    return match page {
        Page::Home => PAGES.lock().unwrap().home.clone(),
        Page::Contact => PAGES.lock().unwrap().contact.clone(),
        Page::Portfolio => PAGES.lock().unwrap().portfolio.clone()
    }
}


pub async fn refresh_portfolio() {
    let ctx = portfolio::get_context();
    let rendered_page = TEMPLATES.render("portfolio.html", &ctx).unwrap();
    PAGES.lock().unwrap().portfolio = rendered_page;
}