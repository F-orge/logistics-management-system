use std::collections::BTreeMap;

use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sea_orm::{ActiveEnum, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use tonic::{Response, Status};

use crate::{
    models::{
        _entities::user::{Column, Entity},
        _proto::auth::{
            auth_service_server::{AuthService as GrpcAuthService, AuthServiceServer},
            AuthResponse,
        },
    },
    AppState,
};

#[derive(Default)]
pub struct AuthService {
    db: DatabaseConnection,
    issuer: String,
}

impl AuthService {
    pub fn new(db: &DatabaseConnection, issuer: String) -> AuthServiceServer<AuthService> {
        AuthServiceServer::new(Self {
            db: db.clone(),
            issuer,
        })
    }
}

#[tonic::async_trait]
impl GrpcAuthService for AuthService {
    async fn login(
        &self,
        request: tonic::Request<crate::models::_proto::auth::AuthRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::models::_proto::auth::AuthResponse>,
        tonic::Status,
    > {
        let host_name = match request.metadata().get("host") {
            Some(host_name) => match host_name.to_str() {
                Ok(host_name) => host_name.to_string(),
                Err(_) => return Err(Status::invalid_argument("Invalid `Hostname` header")),
            },
            None => return Err(Status::invalid_argument("missing `Hostname` header")),
        };

        let payload = request.into_inner();

        let user = match Entity::find()
            .filter(Column::Email.eq(payload.email))
            //TODO: convert this to hash before sending it to database. it must match algorithm used in the database for hashing.
            .filter(Column::Password.eq(payload.password))
            .one(&self.db)
            .await
        {
            Ok(model) => match model {
                Some(model) => model,
                None => return Err(Status::not_found("User does not exists")),
            },
            Err(err) => match err {
                sea_orm::DbErr::RecordNotFound(_) => {
                    return Err(Status::invalid_argument("Invalid email or password"))
                }
                _ => return Err(Status::internal("Internal server error")),
            },
        };

        // TODO: change this to be environment variable
        let key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-random-key") {
            Ok(value) => value,
            Err(err) => {
                tracing::error!("encryption key error: {}", err);
                return Err(Status::internal("Internal server error"));
            }
        };

        let mut claims = BTreeMap::new();

        let expiration = (Utc::now() + std::time::Duration::from_secs(3600)).to_string();

        // reserve claims
        claims.insert("iss", self.issuer.to_owned());
        claims.insert("sub", user.id.to_string());

        // TODO: implement aud (audience) claims using hostname

        claims.insert("aud", host_name);
        claims.insert("exp", expiration.clone());
        claims.insert("iat", Utc::now().to_string());
        claims.insert("jti", Uuid::new_v4().to_string());
        claims.insert("nbf", Utc::now().to_string());

        // private claims
        claims.insert("role", user.user_role);
        claims.insert("email", user.email);

        let token = match claims.sign_with_key(&key) {
            Ok(token) => token,
            Err(err) => {
                tracing::error!("jwt token error {}", err);
                return Err(Status::internal("Internal server error"));
            }
        };
        Ok(Response::new(AuthResponse { token }))
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
    use sqlx::{pool::PoolOptions, ConnectOptions, Postgres};
    use tonic::{transport::Server, Request};

    use super::*;

    use crate::{
        controllers::user::UserService,
        models::_proto::{
            auth::{auth_service_client::AuthServiceClient, AuthRequest},
            employee_management::{
                user_service_client::UserServiceClient, InsertUserRequest, Role,
            },
        },
        utils::test::start_server,
    };

    #[sqlx::test]
    #[test_log::test]
    async fn test_auth_login(_pool: PoolOptions<Postgres>, options: impl ConnectOptions) {
        let db = Database::connect(options.to_url_lossy()).await.unwrap();
        let (_, channel) = start_server(
            Server::builder()
                .add_service(UserService::new(&db))
                .add_service(AuthService::new(&db, "api.f-org-e.systems".into())),
        )
        .await;

        let mut user_client = UserServiceClient::new(channel.clone());
        let mut auth_client =
            AuthServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
                req.metadata_mut()
                    .insert("host", "www.example.com".parse().unwrap());
                Ok(req)
            });

        let request = InsertUserRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
            role: Role::Admin.into(),
        };

        // ignore it
        let _ = user_client.insert_user(request).await;

        let auth_request = AuthRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
        };

        let response = auth_client.login(auth_request).await;

        assert!(response.is_ok(), "{:#?}", response.err());
    }
}
