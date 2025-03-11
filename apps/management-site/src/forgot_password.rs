use axum::{
    Router,
    http::HeaderValue,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use axum_extra::extract::Form;
use lib_core::{AppState, error::AskamaResult};
use rinja::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "pages/forgot-password/step-1.html.jinja")]
struct ForgotPasswordPage;

async fn show_step_1() -> ForgotPasswordPage {
    ForgotPasswordPage {}
}

#[derive(Debug, Deserialize)]
struct ForgotPasswordDTO {
    email: String,
}

async fn submit_step_1(Form(form): Form<ForgotPasswordDTO>) -> AskamaResult<Redirect> {
    println!("{:#?}", form);

    // TODO: submit a 6 pin code via email. save the code in the database

    Ok(Redirect::to("/forgot-password/confirm"))
}

#[derive(Template)]
#[template(path = "pages/forgot-password/step-2.html.jinja")]
struct EmailCodePage;

async fn show_step_2() -> EmailCodePage {
    EmailCodePage {}
}

#[derive(Debug, Deserialize)]
struct EmailCodeDTO {
    code: Vec<u16>,
}

async fn submit_step_2(Form(form): Form<EmailCodeDTO>) -> AskamaResult<Redirect> {
    println!("{:#?}", form);
    Ok(Redirect::to("/forgot-password/reset"))
}

#[derive(Template)]
#[template(path = "pages/forgot-password/step-3.html.jinja")]
struct ResetPasswordPage;

async fn show_step_3() -> ResetPasswordPage {
    ResetPasswordPage {}
}

#[derive(Debug, Deserialize)]
struct ResetPasswordDTO {
    password: String,
    confirm_password: String,
}

async fn submit_step_3(Form(form): Form<ResetPasswordDTO>) -> AskamaResult<Redirect> {
    println!("{:#?}", form);
    Ok(Redirect::to("/login"))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(show_step_1))
        .route("/", post(submit_step_1))
        .route("/confirm", get(show_step_2))
        .route("/confirm", post(submit_step_2))
        .route("/reset", get(show_step_3))
        .route("/reset", post(submit_step_3))
}
