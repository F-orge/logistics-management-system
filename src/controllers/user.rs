use futures::TryStreamExt;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    ModelTrait, QueryFilter, Set, TryIntoModel,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::models::_proto::employee_management::{
    get_user_request::Identifier,
    user_service_server::{UserService as GrpcUserService, UserServiceServer},
    DeleteUserRequest, Empty, GetUserRequest, InsertUserRequest, Role, UpdateUserEmailRequest,
    UpdateUserPasswordRequest, UpdateUserRoleRequest, User,
};

use crate::models::_entities::sea_orm_active_enums::RoleEnum;
use crate::models::_entities::user::{ActiveModel, Column, Entity};

#[derive(Debug, Default)]
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: &DatabaseConnection) -> UserServiceServer<Self> {
        UserServiceServer::new(Self { db: db.clone() })
    }
}

#[tonic::async_trait]
impl GrpcUserService for UserService {
    type GetUsersStream = ReceiverStream<Result<User, Status>>;

    async fn insert_user(
        &self,
        request: Request<InsertUserRequest>,
    ) -> Result<Response<User>, Status> {
        let payload: Result<ActiveModel, _> = request.into_inner().try_into();

        let db_response = match payload {
            Ok(value) => value.insert(&self.db).await,
            Err(_) => return Err(Status::invalid_argument("Invalid data")),
        };

        match db_response {
            Ok(model) => match model.try_into() {
                Ok(response) => Ok(Response::new(response)),
                Err(_) => Err(Status::internal("Internal server error")),
            },
            Err(err) => match err {
                DbErr::RecordNotInserted => Err(Status::internal("Record not inserted")),
                _ => Err(Status::internal("Internal server error")),
            },
        }
    }

    async fn get_users(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<Self::GetUsersStream>, Status> {
        let payload = request.into_inner();

        let db_response = match payload.identifier {
            Some(value) => match value {
                Identifier::Role(role) => match role {
                    x if x == RoleEnum::SuperAdmin as i32 => {
                        Entity::find()
                            .filter(Column::Role.eq(RoleEnum::SuperAdmin))
                            .stream(&self.db)
                            .await
                    }
                    x if x == RoleEnum::Admin as i32 => {
                        Entity::find()
                            .filter(Column::Role.eq(RoleEnum::Admin))
                            .stream(&self.db)
                            .await
                    }
                    x if x == RoleEnum::Employee as i32 => {
                        Entity::find()
                            .filter(Column::Role.eq(RoleEnum::Employee))
                            .stream(&self.db)
                            .await
                    }
                    x if x == RoleEnum::Client as i32 => {
                        Entity::find()
                            .filter(Column::Role.eq(RoleEnum::Client))
                            .stream(&self.db)
                            .await
                    }
                    _ => return Err(Status::invalid_argument("Invalid role selected")),
                },
                _ => return Err(Status::invalid_argument("Invalid identifier")),
            },
            None => return Err(Status::invalid_argument("Identifier required")),
        };

        match db_response {
            Ok(mut stream) => {
                let (tx, rx) = tokio::sync::mpsc::channel(4);
                while let Ok(item) = stream.try_next().await {
                    if let Some(item) = item {
                        let item = match item.try_into_model() {
                            Ok(model) => match model.try_into() {
                                Ok(user) => user,
                                // TODO: should we continue?? or break the stream?
                                Err(_) => continue,
                            },
                            Err(_err) => {
                                return Err(Status::internal("Cannot convert item into model"))
                            }
                        };
                        match tx.send(Ok(item)).await {
                            Ok(_) => {}
                            Err(_) => {
                                return Err(Status::internal("Unable to send item to channel"))
                            }
                        }
                    }
                }
                Ok(Response::new(rx.into()))
            }
            Err(err) => match err {
                DbErr::RecordNotFound(data) => Err(Status::not_found(data)),
                _ => Err(Status::internal("Internal server error")),
            },
        }
    }

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let payload = request.into_inner();

        let identifier = match payload.identifier {
            Some(value) => value,
            None => return Err(Status::invalid_argument("Invalid argument")),
        };

        let db_response = match identifier {
            Identifier::Email(email) => {
                Entity::find()
                    .filter(Column::Email.eq(email))
                    .one(&self.db)
                    .await
            }
            Identifier::Id(id) => {
                let uuid = match id.parse::<sea_orm::prelude::Uuid>() {
                    Ok(parsed_id) => parsed_id,
                    Err(_) => return Err(Status::invalid_argument("Invalid UUID")),
                };
                Entity::find_by_id(uuid).one(&self.db).await
            }
            _ => return Err(Status::invalid_argument("Invalid argument")),
        };

        match db_response {
            Ok(value) => match value {
                Some(model) => match model.try_into() {
                    Ok(user) => Ok(Response::new(user)),
                    Err(_) => Err(Status::internal("Cannot convert model into response")),
                },
                None => Err(Status::not_found("Not found")),
            },
            Err(_err) => Err(Status::internal("Internal server error")),
        }
    }
    async fn update_user_email(
        &self,
        request: Request<UpdateUserEmailRequest>,
    ) -> Result<Response<User>, Status> {
        let payload = request.into_inner();

        let primary_key: sea_orm::prelude::Uuid = match payload.id.parse() {
            Ok(value) => value,
            Err(_err) => return Err(Status::invalid_argument("Invalid UUID")),
        };

        let mut current_user = match Entity::find_by_id(primary_key)
            .filter(Column::Email.eq(payload.current_email))
            .one(&self.db)
            .await
        {
            Ok(value) => match value {
                Some(value) => value.into_active_model(),
                None => return Err(Status::not_found("User not found")),
            },
            Err(err) => match err {
                DbErr::RecordNotFound(data) => {
                    return Err(Status::not_found(format!("User not found {}", data)))
                }
                _ => return Err(Status::internal("Internal server error")),
            },
        };

        current_user.email = Set(payload.new_email);

        match current_user.update(&self.db).await {
            Ok(model) => match model.try_into() {
                Ok(updated_user) => Ok(Response::new(updated_user)),
                Err(_) => Err(Status::internal(
                    "Unable to convert updated_user to response",
                )),
            },
            Err(err) => Err(Status::internal("Internal server error")),
        }
    }

    async fn update_user_password(
        &self,
        request: Request<UpdateUserPasswordRequest>,
    ) -> Result<Response<User>, Status> {
        let payload = request.into_inner();

        let primary_key: sea_orm::prelude::Uuid = match payload.id.parse() {
            Ok(value) => value,
            Err(_err) => return Err(Status::invalid_argument("Invalid uuid")),
        };

        let mut current_user = match Entity::find_by_id(primary_key)
            .filter(Column::Password.eq(payload.current_password))
            .one(&self.db)
            .await
        {
            Ok(value) => match value {
                Some(value) => value.into_active_model(),
                None => return Err(Status::not_found("User not found")),
            },
            Err(err) => match err {
                DbErr::RecordNotFound(_data) => return Err(Status::not_found("User not found")),
                _ => return Err(Status::internal("Internal server error")),
            },
        };

        current_user.password = Set(payload.new_password);

        match current_user.update(&self.db).await {
            Ok(model) => match model.try_into() {
                Ok(updated_user) => Ok(Response::new(updated_user)),
                Err(_) => Err(Status::internal(
                    "Unable to convert updated_user to response",
                )),
            },
            Err(err) => Err(Status::internal("Internal server error")),
        }
    }
    async fn update_user_role(
        &self,
        request: Request<UpdateUserRoleRequest>,
    ) -> Result<Response<User>, Status> {
        let payload = request.into_inner();

        let primary_key: sea_orm::prelude::Uuid = match payload.id.parse() {
            Ok(value) => value,
            Err(_err) => return Err(Status::invalid_argument("Invalid UUID")),
        };

        let mut current_user = match Entity::find_by_id(primary_key).one(&self.db).await {
            Ok(value) => match value {
                Some(value) => value.into_active_model(),
                None => return Err(Status::not_found("User not found")),
            },
            Err(err) => match err {
                DbErr::RecordNotFound(_) => return Err(Status::not_found("User not found")),
                _ => return Err(Status::internal("Internal server error")),
            },
        };

        current_user.role = match payload.role {
            x if x == Role::SuperAdmin as i32 => Set(RoleEnum::SuperAdmin),
            x if x == Role::Admin as i32 => Set(RoleEnum::Admin),
            x if x == Role::Employee as i32 => Set(RoleEnum::Employee),
            x if x == Role::Client as i32 => Set(RoleEnum::Client),
            _ => return Err(Status::invalid_argument("Invalid role")),
        };

        match current_user.update(&self.db).await {
            Ok(model) => match model.try_into() {
                Ok(updated_user) => Ok(Response::new(updated_user)),
                Err(_) => Err(Status::internal(
                    "Unable to convert updated_user to response",
                )),
            },
            Err(err) => Err(Status::internal("Internal server error")),
        }
    }
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<Empty>, Status> {
        let payload = request.into_inner();

        let primary_key: sea_orm::prelude::Uuid = match payload.id.parse() {
            Ok(value) => value,
            Err(_err) => return Err(Status::invalid_argument("Invalid UUID")),
        };

        let db_response = match Entity::find_by_id(primary_key).one(&self.db).await {
            Ok(value) => match value {
                Some(value) => value.delete(&self.db).await,
                None => return Err(Status::not_found("User not found")),
            },
            Err(_err) => return Err(Status::internal("Internal server error")),
        };

        match db_response {
            Ok(_value) => Ok(Response::new(Empty {})),
            Err(_) => Err(Status::internal("Internal server error")),
        }
    }
}

#[cfg(test)]
mod test {

    #![allow(clippy::unwrap_used)]

    use migration::MigratorTrait;
    use sea_orm::Database;
    use sqlx::{pool::PoolOptions, ConnectOptions, Postgres};
    use tonic::transport::Server;

    use crate::{
        models::_proto::employee_management::{user_service_client::UserServiceClient, Role},
        utils::test::start_server,
    };

    use super::*;

    #[sqlx::test]
    #[test_log::test]
    async fn test_insert_user(_pool: PoolOptions<Postgres>, options: impl ConnectOptions) {
        let db = Database::connect(options.to_url_lossy()).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let (_, channel) = start_server(Server::builder().add_service(UserService::new(&db))).await;

        let mut client = UserServiceClient::new(channel);

        let request = InsertUserRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
            role: Role::Admin.into(),
        };

        let response = client.insert_user(request).await;

        assert!(response.is_ok());

        let response = response.unwrap();

        let payload = response.into_inner();
        assert_eq!(payload.email, "johndoe@gmail.com");
        assert_eq!(payload.role, Role::Admin as i32);
    }

    #[sqlx::test]
    #[test_log::test]
    async fn test_get_user_by_id(_pool: PoolOptions<Postgres>, options: impl ConnectOptions) {
        let db = Database::connect(options.to_url_lossy()).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let (_, channel) = start_server(Server::builder().add_service(UserService::new(&db))).await;

        let mut client = UserServiceClient::new(channel);

        let request = InsertUserRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
            role: Role::Admin.into(),
        };

        let response = client.insert_user(request).await.unwrap();

        let payload = response.into_inner();

        let request = GetUserRequest {
            identifier: Some(Identifier::Id(payload.id)),
        };

        let response = client.get_user(request).await;

        assert!(response.is_ok());

        let payload = response.unwrap().into_inner();

        assert_eq!(payload.email, "johndoe@gmail.com");
        assert_eq!(payload.role, Role::Admin as i32);
    }

    #[sqlx::test]
    #[test_log::test]
    async fn test_get_user_by_email(_pool: PoolOptions<Postgres>, options: impl ConnectOptions) {
        let db = Database::connect(options.to_url_lossy()).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let (_, channel) = start_server(Server::builder().add_service(UserService::new(&db))).await;

        let mut client = UserServiceClient::new(channel);

        let request = InsertUserRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
            role: Role::Admin.into(),
        };

        let response = client.insert_user(request).await.unwrap();

        let payload = response.into_inner();

        let request = GetUserRequest {
            identifier: Some(Identifier::Email(payload.email)),
        };

        let response = client.get_user(request).await;

        assert!(response.is_ok());

        let payload = response.unwrap().into_inner();

        assert_eq!(payload.email, "johndoe@gmail.com");
        assert_eq!(payload.role, Role::Admin as i32);
    }

    #[sqlx::test]
    #[test_log::test]
    async fn test_update_user_email(_pool: PoolOptions<Postgres>, options: impl ConnectOptions) {
        let db = Database::connect(options.to_url_lossy()).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let (_, channel) = start_server(Server::builder().add_service(UserService::new(&db))).await;

        let mut client = UserServiceClient::new(channel);

        let request = InsertUserRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
            role: Role::Admin.into(),
        };

        let response = client.insert_user(request).await.unwrap();

        let payload = response.into_inner();

        let request = UpdateUserEmailRequest {
            id: payload.id,
            current_email: "johndoe@gmail.com".into(),
            new_email: "johndoe_new@gmail.com".into(),
        };

        let response = client.update_user_email(request).await;

        let payload = response.unwrap().into_inner();

        assert_eq!(payload.email, "johndoe_new@gmail.com");
        assert_eq!(payload.role, Role::Admin as i32);
    }

    #[sqlx::test]
    #[test_log::test]
    async fn test_update_user_password(_pool: PoolOptions<Postgres>, options: impl ConnectOptions) {
        let db = Database::connect(options.to_url_lossy()).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let (_, channel) = start_server(Server::builder().add_service(UserService::new(&db))).await;

        let mut client = UserServiceClient::new(channel);

        let request = InsertUserRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
            role: Role::Admin.into(),
        };

        let response = client.insert_user(request).await.unwrap();

        let payload = response.into_inner();

        let request = UpdateUserPasswordRequest {
            id: payload.id,
            current_password: "johndoepassword".into(),
            new_password: "johndoenewpassowrd".into(),
        };

        let response = client.update_user_password(request).await;

        let payload = response.unwrap().into_inner();

        assert_eq!(payload.email, "johndoe@gmail.com");
        assert_eq!(payload.role, Role::Admin as i32);
    }

    #[sqlx::test]
    #[test_log::test]
    async fn test_update_user_role(_pool: PoolOptions<Postgres>, options: impl ConnectOptions) {
        let db = Database::connect(options.to_url_lossy()).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let (_, channel) = start_server(Server::builder().add_service(UserService::new(&db))).await;

        let mut client = UserServiceClient::new(channel);

        let request = InsertUserRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
            role: Role::Admin.into(),
        };

        let response = client.insert_user(request).await.unwrap();

        let payload = response.into_inner();

        let request = UpdateUserRoleRequest {
            id: payload.id,
            role: Role::SuperAdmin as i32,
        };

        let response = client.update_user_role(request).await;

        let payload = response.unwrap().into_inner();

        assert_eq!(payload.email, "johndoe@gmail.com");
        assert_eq!(payload.role, Role::SuperAdmin as i32);
    }

    #[sqlx::test]
    #[test_log::test]
    async fn test_delete_user(_pool: PoolOptions<Postgres>, options: impl ConnectOptions) {
        let db = Database::connect(options.to_url_lossy()).await.unwrap();
        migration::Migrator::up(&db, None).await.unwrap();

        let (_, channel) = start_server(Server::builder().add_service(UserService::new(&db))).await;

        let mut client = UserServiceClient::new(channel);

        let request = InsertUserRequest {
            email: "johndoe@gmail.com".into(),
            password: "johndoepassword".into(),
            role: Role::Admin.into(),
        };

        let response = client.insert_user(request).await.unwrap();

        let payload = response.into_inner();

        let request = DeleteUserRequest { id: payload.id };

        let response = client.delete_user(request).await;

        assert!(response.is_ok());
    }
}
