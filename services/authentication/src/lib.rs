use sqlx::Row;
use sqlx::{Pool, Postgres};
use tonic::{IntoRequest, Response, Status};

use crate_proto::auth::{
    AuthBasicLoginRequest, AuthBasicRegisterRequest, AuthResponse,
    auth_service_server::{AuthService as GrpcAuthService, AuthServiceServer},
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
        request: tonic::Request<AuthBasicLoginRequest>,
    ) -> std::result::Result<tonic::Response<AuthResponse>, tonic::Status> {
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
        request: tonic::Request<AuthBasicRegisterRequest>,
    ) -> std::result::Result<tonic::Response<AuthResponse>, tonic::Status> {
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

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use sqlx::{Pool, Postgres};
    use tonic::transport::Server;

    use super::*;

    use crate_proto::auth::{AuthBasicRegisterRequest, auth_service_client::AuthServiceClient};
    use crate_utils::test::start_server;

    #[sqlx::test(migrations = "../../migrations")]
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

        assert!(response.is_ok(), "{:?}", response.err());
    }
}
