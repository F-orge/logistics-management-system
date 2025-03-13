use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use lib_core::error::ErrorResponse;

pub fn routes() -> OpenApiRouter<lib_core::AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(refresh))
}

#[derive(Debug, Deserialize, ToSchema)]
struct LoginDTO {
    #[schema(example = "johndoe@email.com")]
    email: String,
    #[schema(example = "RandomPassword1")]
    password: String,
}

#[derive(Debug,Deserialize,ToSchema)]
struct AccessTokenDTO {
    #[schema(example = "<ACCESS_TOKEN>")]
    access_token:String,
    #[schema(example = "<REFRESH_TOKEN>")]
    refresh_token:String,
    #[schema(example = "Bearer")]
    token_type:String,
    #[schema(example = 3600)]
    exp:u16,
    #[schema(example = json!(["service:read","service:write"]))]
    scopes:Vec<String>
}

#[utoipa::path(
    post, 
    tag = "Authentication", 
    path = "/login",
    request_body(content = LoginDTO, content_type = "application/x-www-form-urlencoded"),
    responses(
        (
            status = 200, 
            description = "Successfully Logged In",  
            body = AccessTokenDTO
        ),
        (status = 400, description = "Invalid email or password"),
        (status = 500, description = "Internal server error")
    )
)]
async fn login() {}

#[derive(Debug,Deserialize,ToSchema)]
struct RefreshTokenDTO {
    #[schema(example = "<REFRESH_TOKEN>")]
    refresh_token:String,
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
            body = AccessTokenDTO
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
