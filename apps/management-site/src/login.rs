use askama::Template;
use axum::{
    Form, Router,
    response::Redirect,
    routing::{get, post},
};
use garde::Validate;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "pages/login.html.jinja")]
struct ShowTemplate {
    email_error: Option<String>,
    password_error: Option<String>,
}

async fn show() -> ShowTemplate {
    ShowTemplate {
        email_error: None,
        password_error: None,
    }
}

#[derive(Debug, Deserialize, Validate)]
struct SubmitDTO {
    #[garde(email)]
    email: String,
    #[garde(length(min = 8))]
    password: String,
}

async fn submit(Form(form): Form<SubmitDTO>) -> Result<Redirect, ShowTemplate> {
    form.validate().map_err(|err| {
        let (mut email_error, mut password_error) = (None, None);
        for (path, err) in err.iter() {
            match path.to_string().as_str() {
                "email" => email_error = Some(err.message().to_string()),
                "password" => password_error = Some(err.message().to_string()),
                _ => {}
            }
        }
        ShowTemplate {
            email_error,
            password_error,
        }
    })?;

    println!("{} {}", form.email, form.password);

    Ok(Redirect::permanent("/"))
}

pub fn routes() -> Router {
    Router::new().route("/", get(show)).route("/", post(submit))
}
