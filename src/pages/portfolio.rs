use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    image: String,
    title: String,
    text: String,
    technologies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JSON {
    portfolio_data: Vec<Project>
}

fn update_projects() -> Vec<Project> {
    let data = ureq::get("https://raw.githubusercontent.com/alvidrezluke/portfolio-v2/main/portfolio-files/portfolio.json").call().unwrap().into_string().expect("Unable to stringify response");
    let json: JSON = serde_json::from_str(&data).unwrap();
    json.portfolio_data
}

pub fn get_context() -> Context {
    let mut ctx = Context::new();
    ctx.insert("projects", &update_projects());
    ctx
}