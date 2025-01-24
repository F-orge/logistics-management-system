use std::collections::BTreeMap;

use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::Row;
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    Pool, Postgres,
};
use tonic::{IntoRequest, Response, Status};

use crate::{
    models::{
        _entities::user::{Column, Entity},
        _proto::auth::{
            auth_service_server::{AuthService as GrpcAuthService, AuthServiceServer},
            AuthBasicLoginRequest, AuthResponse,
        },
    },
    AppState,
};

pub struct AuthService {
    db: Pool<Postgres>,
}

impl AuthService {
    pub fn new(db: &Pool<Postgres>) -> AuthServiceServer<AuthService> {
        AuthServiceServer::new(Self { db: db.clone() })
    }
}

#[tonic::async_trait]
impl GrpcAuthService for AuthService {
    async fn basic_login(
        &self,
        request: tonic::Request<crate::models::_proto::auth::AuthBasicLoginRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::models::_proto::auth::AuthResponse>,
        tonic::Status,
    > {
        let payload = request.into_inner();

        let token: String = match sqlx::query("select \"auth\".\"basic_login\" ($1,$2)")
            .bind(payload.email)
            .bind(payload.password)
            .fetch_one(&self.db)
            .await
        {
            Ok(row) => row.get(0),
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::invalid_argument("Invalid username or password"));
            }
        };

        Ok(Response::new(AuthResponse {
            access_token: token.clone(),
            refresh_token: token,
            token_type: "bearer".into(),
            expires_in: 3600,
        }))
    }
    async fn basic_register(
        &self,
        request: tonic::Request<crate::models::_proto::auth::AuthBasicRegisterRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::models::_proto::auth::AuthResponse>,
        tonic::Status,
    > {
        let payload = request.into_inner();

        let mut trx = match self.db.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to start transaction"));
            }
        };

        let _ = match sqlx::query(
            "insert into \"auth\".\"basic_user\" (email,password) values ($1,$2)",
        )
        .bind(payload.email.clone())
        .bind(payload.password.clone())
        .execute(&mut *trx)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to store user"));
            }
        };

        let _ = match trx.commit().await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to commit changes"));
            }
        };

        let request = AuthBasicLoginRequest {
            email: payload.email,
            password: payload.password,
        }
        .into_request();

        self.basic_login(request).await
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct JWTReserveClaims {
    #[serde(rename = "iss")]
    issuer: String,
    #[serde(rename = "aud")]
    audience: String,
    #[serde(rename = "sub")]
    subject: Uuid,
    #[serde(rename = "exp")]
    expiry: DateTime<Utc>,
    #[serde(rename = "iat")]
    issued_at: DateTime<Utc>,
    #[serde(rename = "jti")]
    jwt_id: Uuid,
    #[serde(rename = "nbf")]
    not_before_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct JWTPrivateClaims {
    role: i16,
    email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JWTClaims {
    #[serde(flatten)]
    reserve: JWTReserveClaims,
    #[serde(flatten)]
    private: JWTPrivateClaims,
    is_authenticated: bool,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> axum::response::Response {
    // TODO: either find this in cookies or in headers
    let token = match request.headers().get("Authentication") {
        Some(token) => token,
        None => return next.run(request).await,
    };

    let mut claims: JWTClaims = match token.to_str() {
        Ok(token) => match token.verify_with_key(&state.key) {
            Ok(claims) => claims,
            Err(_) => {
                return axum::response::Response::builder()
                    .body(Body::empty())
                    .unwrap_or_default()
            }
        },
        Err(_) => {
            return axum::response::Response::builder()
                .body(Body::empty())
                .unwrap_or_default()
        }
    };

    // check if the token is ready to use
    if claims.reserve.not_before_time > Utc::now() {
        return axum::response::Response::builder()
            .status(403)
            .body(Body::empty())
            .unwrap_or_default();
    }

    let host_name = match request.headers().get("Host") {
        Some(host_name) => match host_name.to_str() {
            Ok(host_name) => host_name.to_string(),
            Err(_) => {
                return axum::response::Response::builder()
                    .status(403)
                    .body(Body::empty())
                    .unwrap_or_default()
            }
        },
        None => {
            return axum::response::Response::builder()
                .status(403)
                .body(Body::empty())
                .unwrap_or_default()
        }
    };

    // check if the token is from the aud (audience) using hostname
    if claims.reserve.audience != host_name {
        return axum::response::Response::builder()
            .status(403)
            .body(Body::empty())
            .unwrap_or_default();
    }

    // TODO: convert this to env variable or state since we are not the issuer
    if claims.reserve.issuer != "api.f-org-e.systems" {
        return axum::response::Response::builder()
            .status(403)
            .body(Body::empty())
            .unwrap_or_default();
    }

    // let's check if the token is expired
    if claims.reserve.expiry < Utc::now() {
        return axum::response::Response::builder()
            .status(403)
            .body(Body::empty())
            .unwrap_or_default();
    }

    // Future versions: have a way to check if the token is banned or not using jti claims

    claims.is_authenticated = true;

    request.extensions_mut().insert(claims);

    next.run(request).await
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use sea_orm::Database;
    use sqlx::{pool::PoolOptions, ConnectOptions, Pool, Postgres};
    use tonic::{transport::Server, Request};

    use super::*;

    use crate::{
        controllers::user::UserService,
        models::_proto::{
            auth::{
                auth_service_client::AuthServiceClient, AuthBasicLoginRequest,
                AuthBasicRegisterRequest,
            },
            employee_management::{
                user_service_client::UserServiceClient, InsertUserRequest, Role,
            },
        },
        utils::test::start_server,
    };

    #[sqlx::test]
    async fn test_auth_basic_register_and_login(db: Pool<Postgres>) {
        let _ = sqlx::query("alter database postgres set \"app.jwt_secret\" = \"randompassword\"")
            .execute(&db)
            .await
            .unwrap();

        let (_, channel) = start_server(Server::builder().add_service(AuthService::new(&db))).await;

        let mut client = AuthServiceClient::new(channel);

        let request = AuthBasicRegisterRequest {
            email: "sample@email.com".into(),
            password: "randompassowrd".into(),
        };

        let response = client.basic_register(request).await;

        assert!(response.is_ok());
    }
}
