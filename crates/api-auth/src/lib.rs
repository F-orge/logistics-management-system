use std::collections::BTreeMap;

use axum::{Form, Json, extract::State};
use chrono::{Duration, Local};
use jwt::SignWithKey;
use lib_core::{
    AppState,
    error::{Error, ErrorResponse},
    result::Result,
};
use lib_entity::{extensions::get_all_permissions, generated::users};
use lib_security::JWTClaim;
use pwhash::bcrypt;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

pub fn routes() -> OpenApiRouter<lib_core::AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(refresh))
}

#[derive(Debug, Deserialize, ToSchema)]
struct LoginRequest {
    #[schema(example = "johndoe@email.com")]
    email: String,
    #[schema(example = "RandomPassword1")]
    password: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct AccessTokenResponse {
    #[schema(example = "<ACCESS_TOKEN>")]
    access_token: String,
    #[schema(example = "<REFRESH_TOKEN>")]
    refresh_token: String,
    #[schema(example = "Bearer")]
    token_type: String,
    #[schema(example = 3600)]
    exp: u16,
    #[schema(example = json!(["service:read","service:write"]))]
    scopes: Vec<String>,
}

#[axum::debug_handler]
#[utoipa::path(
    post, 
    tag = "Authentication", 
    path = "/login",
    request_body(content = LoginRequest, content_type = "application/x-www-form-urlencoded"),
    responses(
        (
            status = 200, 
            description = "Successfully Logged In",  
            body = AccessTokenResponse
        ),
        (status = 400, description = "Invalid email or password"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login(
    State(AppState { db, key, .. }): State<AppState>,
    Form(LoginRequest { email, password }): Form<LoginRequest>,
) -> Result<Json<AccessTokenResponse>> {
    let model = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(&db)
        .await
        .map_err(lib_core::error::Error::SeaOrm)?
        .ok_or(Error::AuthenticationError)?;

    if !bcrypt::verify(password, &model.password) {
        return Err(Error::AuthenticationError);
    }

    let mut claims = JWTClaim {
        issuer: "sample".into(),
        subject: model.id,
        audience: "logistics".into(),
        expiration: (Local::now().to_utc() + Duration::hours(1)).to_string(),
        not_before: (Local::now().to_utc() - Duration::seconds(1)).to_string(),
        issued_at: Local::now().to_utc().to_string(),
        jwt_id: Uuid::new_v4(),
        claims: get_all_permissions(&db, model.id)
            .await
            .map_err(Error::SeaOrm)?,
    };

    let access_token = claims.clone().sign_with_key(&key).unwrap();

    claims.expiration = (Local::now().to_utc() + Duration::hours(6)).to_string();

    let refresh_token = claims.sign_with_key(&key).unwrap();

    Ok(Json(AccessTokenResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".into(),
        exp: 3600,
        scopes: vec![],
    }))
}

#[derive(Debug, Deserialize, ToSchema)]
struct RefreshTokenDTO {
    #[schema(example = "<REFRESH_TOKEN>")]
    refresh_token: String,
}

#[utoipa::path(
    post, 
    tag = "Authentication", 
    path = "/refresh",
    request_body(content = RefreshTokenDTO, content_type = "application/json"),
    responses(
        (
            status = 200, 
            description = "Successfully Refreshed token",  
            body = AccessTokenResponse
        ),
        (   
            status = 400, 
            description = "Invalid refresh token",
            body = ErrorResponse,
        ),
        (
            status = 500, 
            description = "Internal server error",
            body = ErrorResponse,
        )
    )
)]
async fn refresh() {}
