use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Serialize, Deserialize)]
struct Project {
    img_src: String,
    title: String,
    text: String,
    technologies: Vec<String>,
}

fn update_projects() -> Vec<Project> {
    let mut out = vec![];

    // Will be replaced by getting over network
    for i in 0..10 {
        out.push(Project {
            img_src: "".into(),
            title: format!("{}", i),
            text: "".into(),
            technologies: vec![]
        });
    }

    out
}

pub fn get_context() -> Context {
    let mut ctx = Context::new();
    ctx.insert("projects", &update_projects());
    ctx
}