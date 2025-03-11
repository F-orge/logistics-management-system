use axum::{Router, routing::get};
use rinja::Template;

#[derive(Debug, Template)]
#[template(path = "pages/home.html.jinja")]
struct ShowTemplate;

async fn show() -> ShowTemplate {
    ShowTemplate {}
}

pub fn routes() -> Router {
    Router::new().route("/", get(show))
}
