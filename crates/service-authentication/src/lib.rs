use std::collections::BTreeMap;
use std::time::Duration;

use hmac::Hmac;
use jwt::SignWithKey;
use lib_core::error::Error;
use lib_entity::{prelude::*, users};
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use sha2::Sha256;
use tonic::{Response, Status};

use lib_proto::auth::{
    auth_service_server::{AuthService as GrpcAuthService, AuthServiceServer},
    AuthBasicLoginRequest, AuthResponse,
};

pub struct AuthService {
    db: DatabaseConnection,
    encryption_key: Hmac<Sha256>,
}

impl AuthService {
    pub fn new(
        db: &DatabaseConnection,
        encryption_key: Hmac<Sha256>,
    ) -> AuthServiceServer<AuthService> {
        AuthServiceServer::new(Self {
            db: db.clone(),
            encryption_key,
        })
    }
}

#[tonic::async_trait]
impl GrpcAuthService for AuthService {
    async fn basic_login(
        &self,
        request: tonic::Request<AuthBasicLoginRequest>,
    ) -> std::result::Result<tonic::Response<AuthResponse>, tonic::Status> {
        let payload = request.into_inner();

        // TODO: match the user
        let user = Users::find()
            .filter(users::Column::Email.eq(payload.email))
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        // TODO: hash this verify password
        if user.password != payload.password {
            return Err(Status::unauthenticated("Invalid email or password"));
        }

        // TODO: add custom claims
        let claims = lib_security::JWTClaim {
            issuer: "authentication-service".into(),
            subject: user.id,
            audience: "management".into(),
            expiration: (sqlx::types::chrono::Utc::now() + Duration::from_secs(3600)).to_string(),
            not_before: (sqlx::types::chrono::Utc::now() - Duration::from_secs(1)).to_string(),
            issued_at: sqlx::types::chrono::Utc::now().to_string(),
            jwt_id: sqlx::types::Uuid::new_v4(),
            claims: BTreeMap::new(),
        };

        let token = claims
            .sign_with_key(&self.encryption_key)
            .map_err(|_| Status::internal("Cannot encrypt key"))?;

        Ok(Response::new(AuthResponse {
            access_token: token.into(),
            token_type: "bearer".into(),
            expires_in: 3600,
        }))
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use hmac::Mac;
    use jwt::VerifyWithKey;
    use lib_entity::sea_orm_active_enums::AuthType;
    use lib_security::JWTClaim;
    use sea_orm::ActiveModelBehavior;
    use sea_orm::ActiveModelTrait;
    use sea_orm::Database;
    use sea_orm::Set;
    use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, ConnectOptions, Postgres};
    use tonic::transport::Server;

    use lib_core::test::start_server;
    use lib_proto::auth::auth_service_client::AuthServiceClient;

    #[sqlx::test(migrations = "../../migrations")]
    #[tracing_test::traced_test]
    async fn test_auth_basic_login(
        _: PoolOptions<Postgres>,
        conn_options: PgConnectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::connect(conn_options.to_url_lossy()).await?;
        let mut user = users::ActiveModel::new();

        user.email = Set("sample@email.com".into());
        user.password = Set("Randompassword1!".into());
        user.auth_type = Set(AuthType::BasicAuth);

        let _ = user.insert(&db).await?;

        let key = Hmac::new_from_slice(b"secret-key!!")?;

        let (_, channel) =
            start_server(Server::builder().add_service(AuthService::new(&db, key.clone()))).await;

        let mut client = AuthServiceClient::new(channel);

        let request = AuthBasicLoginRequest {
            email: "sample@email.com".into(),
            password: "Randompassword1!".into(),
        };

        let response = client.basic_login(request).await;

        assert!(response.is_ok(), "{:?}", response.err());

        // decrypt the key
        let response: Result<JWTClaim, jwt::Error> =
            response?.into_inner().access_token.verify_with_key(&key);

        assert!(response.is_ok(), "{:?}", response.err());

        Ok(())
    }
}
