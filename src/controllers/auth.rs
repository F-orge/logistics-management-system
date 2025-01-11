use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sea_orm::{ActiveEnum, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sha2::Sha256;
use sqlx::types::chrono::Utc;
use tonic::{Response, Status};

use crate::models::{
    _entities::user::{Column, Entity, Model},
    _proto::auth::{
        auth_service_server::{AuthService as GrpcAuthService, AuthServiceServer},
        AuthResponse,
    },
};

#[derive(Default)]
pub struct AuthService {
    db: DatabaseConnection,
}

impl AuthService {
    pub fn new(db: &DatabaseConnection) -> AuthServiceServer<AuthService> {
        AuthServiceServer::new(Self { db: db.clone() })
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

        claims.insert("sub", user.id.to_string());
        claims.insert("email", user.email);
        claims.insert("role", user.role.to_value().to_string());
        claims.insert("exp", expiration.clone());

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

#[cfg(test)]
mod test {

    use migration::MigratorTrait;
    use sea_orm::Database;
    use sqlx::{pool::PoolOptions, ConnectOptions, Postgres};
    use tonic::transport::Server;

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
        migration::Migrator::up(&db, None).await.unwrap();
        let (_, channel) = start_server(
            Server::builder()
                .add_service(UserService::new(&db))
                .add_service(AuthService::new(&db)),
        )
        .await;

        let mut user_client = UserServiceClient::new(channel.clone());
        let mut auth_client = AuthServiceClient::new(channel);

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

        assert!(response.is_ok());
    }
}
