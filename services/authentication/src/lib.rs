use sqlx::{Pool, Postgres};
use tonic::{IntoRequest, Response, Status};

use crate_proto::auth::{
    auth_service_server::{AuthService as GrpcAuthService, AuthServiceServer},
    AuthBasicLoginRequest, AuthResponse,
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

        let token = match sqlx::query!(
            r#"select auth.basic_user_login($1,$2) as token"#,
            payload.email,
            payload.password
        )
        .fetch_one(&self.db)
        .await
        {
            Ok(row) => match row.token {
                Some(token) => token,
                None => {
                    return Err(Status::invalid_argument("No token available"));
                }
            },
            Err(err) => {
                println!("{}", err);
                return Err(Status::invalid_argument("Invalid email or password"));
            }
        };

        Ok(Response::new(AuthResponse {
            access_token: token.clone(),
            token_type: "bearer".into(),
            expires_in: 3600,
        }))
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use std::process::exit;

    use sqlx::{Executor, Pool, Postgres};
    use tonic::{transport::Server, Request};

    use super::*;

    use crate_proto::auth::auth_service_client::AuthServiceClient;
    use crate_utils::test::start_server;

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_auth_basic_login(db: Pool<Postgres>) {
        let mut trx = match db.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                panic!("{}", err);
            }
        };

        db.execute(
            r#"
                select set_config('app.jwt.secret','secret',false);
                select set_config('app.jwt.issuer',current_user,false);
                select set_config('app.jwt.audience','management',false);
                select set_config('app.jwt.expiry','3600',false);
        "#,
        )
        .await
        .unwrap();

        if let Err(err) = sqlx::query!(
            "insert into auth.basic_user(email,password) values ($1,$2)",
            "sample@email.com",
            "Randompassword1!"
        )
        .execute(&mut *trx)
        .await
        {
            panic!("{}", err);
        }

        if let Err(err) = trx.commit().await {
            panic!("{}", err);
        }

        let (_, channel) = start_server(Server::builder().add_service(AuthService::new(&db))).await;

        let mut client = AuthServiceClient::new(channel);

        let request = AuthBasicLoginRequest {
            email: "sample@email.com".into(),
            password: "Randompassword1!".into(),
        };

        let response = client.basic_login(request).await;

        assert!(response.is_ok(), "{:?}", response.err());
    }
}
