use askama::Template;
use axum::{Router, routing::get};

#[derive(Debug, Template)]
#[template(path = "pages/home.html.jinja")]
struct ShowTemplate;

async fn show() -> ShowTemplate {
    ShowTemplate {}
}

pub fn routes() -> Router {
    Router::new().route("/", get(show))
}
