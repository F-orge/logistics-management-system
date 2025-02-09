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

    async fn basic_update_password(
        &self,
        request: tonic::Request<crate_proto::auth::AuthBasicUpdatePassword>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        // get auth token
        // set auth token to db
        // update password
        let mut trx = match self.db.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Cannot start transaction"));
            }
        };

        let auth_key = match request.metadata().get("authorization") {
            Some(header_val) => match header_val.to_str() {
                Ok(value) => value.to_string(),
                Err(err) => {
                    tracing::error!("{}", err);
                    return Err(Status::invalid_argument("Invalid Authorization key format"));
                }
            },
            None => return Err(Status::unauthenticated("No Authorization Header")),
        };

        let payload = request.into_inner();

        if let Err(err) = sqlx::query!("SELECT set_config('request.jwt', $1, false)", auth_key)
            .fetch_one(&mut *trx)
            .await
        {
            tracing::error!("{}", err);
            return Err(Status::internal("Cannot set JWT Token"));
        }

        let _ = match sqlx::query!(
            r#"select "auth"."basic_update_password"($1,$2,$3)"#,
            payload.email,
            payload.password,
            payload.new_password
        )
        .execute(&mut *trx)
        .await
        {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::invalid_argument("Invalid email or password"));
            }
        };

        let _ = match trx.commit().await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Cannot commit transaction"));
            }
        };

        Ok(Response::new(()))
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use sqlx::{Pool, Postgres};
    use tonic::{Request, transport::Server};

    use super::*;

    use crate_proto::auth::{
        AuthBasicRegisterRequest, AuthBasicUpdatePassword, auth_service_client::AuthServiceClient,
    };
    use crate_utils::test::start_server;

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_auth_basic_register_and_login(db: Pool<Postgres>) {
        let mut trx = match db.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                tracing::error!("{}", err);
                return;
            }
        };

        let _ = sqlx::query("select set_config('app.jwt_secret','randompassword',false);")
            .execute(&mut *trx)
            .await
            .unwrap();

        let _ = match trx.commit().await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("{}", err);
                return;
            }
        };

        let (_, channel) = start_server(Server::builder().add_service(AuthService::new(&db))).await;

        let mut client = AuthServiceClient::new(channel);

        let request = AuthBasicRegisterRequest {
            email: "sample@email.com".into(),
            password: "randompassowrd".into(),
        };

        let response = client.basic_register(request).await;

        assert!(response.is_ok(), "{:?}", response.err());
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_auth_basic_update_password(db: Pool<Postgres>) {
        let mut trx = match db.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                tracing::error!("{}", err);
                return;
            }
        };

        let _ = sqlx::query("select set_config('app.jwt_secret','randompassword',false);")
            .execute(&mut *trx)
            .await
            .unwrap();

        let _ = match trx.commit().await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("{}", err);
                return;
            }
        };

        let (_, channel) = start_server(Server::builder().add_service(AuthService::new(&db))).await;

        let mut client = AuthServiceClient::new(channel);

        let request = AuthBasicRegisterRequest {
            email: "sample@email.com".into(),
            password: "randompassword".into(),
        };

        let response = client.basic_register(request).await;

        assert!(response.is_ok(), "{:?}", response.err());

        let response = response.unwrap().into_inner();

        let mut request = Request::new(AuthBasicUpdatePassword {
            email: "sample@email.com".into(),
            password: "randompassword".into(),
            new_password: "newest_password!".into(),
        });

        request
            .metadata_mut()
            .append("authorization", response.access_token.parse().unwrap());

        let response = client.basic_update_password(request).await;

        assert!(response.is_ok(), "{:?}", response.err());
    }
}
