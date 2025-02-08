use askama::Template;
use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{Html, Redirect},
};
use crate_proto::auth::AuthBasicLoginRequest;
use garde::Validate;
use serde::Deserialize;
use tonic::Code;

use crate::AuthenticationExtension;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    action_url: String,
}

pub async fn login(
    state: State<AuthenticationExtension>,
) -> Result<Html<String>, (StatusCode, String)> {
    // TODO: check if user is already logged in. use cookie to verify it

    let html = LoginTemplate {
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
pub struct LoginActionPayload {
    #[garde(email)]
    email: String,
    #[garde(length(min = 8))]
    password: String,
}

pub async fn login_action(
    mut state: State<AuthenticationExtension>,
    payload: Form<LoginActionPayload>,
) -> Result<Redirect, (StatusCode, String)> {
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

    // TODO: replace this to be dynamic destination URL since this will be a extension
    Ok(Redirect::permanent(&state.destination_url))
}
