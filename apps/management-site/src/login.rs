use axum::{
    Form, Router,
    extract::State,
    response::Redirect,
    routing::{get, post},
};
use garde::Validate;
use lib_core::{
    AppState,
    error::{AskamaError, AskamaResult, Error},
};
use lib_entity::{prelude::*, users};
use rinja::Template;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "pages/login.html.jinja")]
struct ShowTemplate;

async fn show() -> ShowTemplate {
    ShowTemplate {}
}

#[derive(Debug, Deserialize, Validate)]
struct SubmitDTO {
    #[garde(email)]
    email: String,
    #[garde(length(min = 8))]
    password: String,
}

async fn submit(
    State(state): State<AppState>,
    Form(form): Form<SubmitDTO>,
) -> AskamaResult<Redirect> {
    form.validate().map_err(|err| Error::Garde(err))?;

    let model = Users::find()
        .filter(users::Column::Email.eq(form.email))
        .one(&state.db)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;

    if model.password != form.password {
        return Err(Error::RowNotFound.into());
    }

    Ok(Redirect::permanent("/"))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(show)).route("/", post(submit))
}
