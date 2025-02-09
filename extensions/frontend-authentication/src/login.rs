use askama::Template;
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, Redirect},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use crate_proto::auth::AuthBasicLoginRequest;
use garde::Validate;
use serde::Deserialize;
use tonic::Code;

use crate::AuthenticationExtension;

#[derive(Template)]
#[template(path = "login.html.jinja2")]
pub struct PageTemplate {
    action_url: String,
}

pub async fn page(
    state: State<AuthenticationExtension>,
) -> Result<Html<String>, (StatusCode, String)> {
    // TODO: check if user is already logged in. use cookie to verify it

    let html = PageTemplate {
        action_url: state.action_url.clone(),
    }
    .render();

    match html {
        Ok(html) => Ok(Html(html)),
        Err(err) => {
            tracing::error!("{}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ))
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct SubmitPayload {
    #[garde(email)]
    email: String,
    #[garde(length(min = 8))]
    password: String,
}

pub async fn submit(
    jar: CookieJar,
    mut state: State<AuthenticationExtension>,
    payload: Form<SubmitPayload>,
) -> Result<(CookieJar, Redirect), (StatusCode, String)> {
    if let Err(err) = payload.validate() {
        let mut err_string = String::new();
        for (path, e) in err.into_inner() {
            err_string += &format!("{}: {};", path, e);
        }
        return Err((StatusCode::BAD_REQUEST, err_string));
    }
    let grpc_request = AuthBasicLoginRequest {
        email: payload.email.clone(),
        password: payload.password.clone(),
    };
    let response = match state.grpc_client.basic_login(grpc_request).await {
        Ok(response) => response.into_inner(),
        Err(err) => match err.code() {
            Code::InvalidArgument => {
                return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".into()));
            }
            _ => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                ));
            }
        },
    };

    // TODO: convert the response into a JWT Token. use then `access_token` as the subject in the JWT. save this to cookie.
    let cookie = Cookie::build((
        "Authorization",
        format!("{} {}", response.token_type, response.access_token),
    ))
    .secure(true)
    .path("/")
    .http_only(true)
    .build();

    Ok((jar.add(cookie), Redirect::permanent(&state.destination_url)))
}
