use lib_core::error::Error;
use lib_entity::{prelude::*, users};
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use tonic::{Response, Status};

use lib_proto::auth::{
    auth_service_server::{AuthService as GrpcAuthService, AuthServiceServer},
    AuthBasicLoginRequest, AuthResponse,
};

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

        // verify the password
        if user.password != payload.password {
            return Err(Status::unauthenticated("Invalid email or password"));
        }

        // Todo: generate token
        let token = "todo-token";

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
    use lib_entity::sea_orm_active_enums::AuthType;
    use sea_orm::ActiveModelBehavior;
    use sea_orm::Database;
    use sea_orm::Set;
    use sea_orm::{ActiveModelTrait, EntityTrait};
    use sqlx::{
        pool::PoolOptions,
        postgres::PgConnectOptions,
        types::{chrono::Local, Uuid},
        ConnectOptions, Pool, Postgres,
    };
    use tonic::{metadata::MetadataMap, transport::Server};

    use lib_proto::auth::auth_service_client::AuthServiceClient;
    use lib_utils::test::start_server;

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

        let (_, channel) = start_server(Server::builder().add_service(AuthService::new(&db))).await;

        let mut client = AuthServiceClient::new(channel);

        let request = AuthBasicLoginRequest {
            email: "sample@email.com".into(),
            password: "Randompassword1!".into(),
        };

        let response = client.basic_login(request).await;

        assert!(response.is_ok(), "{:?}", response.err());

        Ok(())
    }
}
