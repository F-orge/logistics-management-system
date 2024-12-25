use tonic::{Request, Response, Status};

use crate::models::employee_management::{user_service_server::{UserService as GrpcUserService, UserServiceServer}, DeleteUserRequest, Empty, GetUserRequest, InsertUserRequest, UpdateUserEmailRequest, UpdateUserPasswordRequest, UpdateUserRoleRequest, User};



#[derive(Debug, Default)]
pub struct UserService;

impl UserService {
    pub fn new() -> UserServiceServer<UserService> {
        UserServiceServer::new(Self::default())
    }
}


#[tonic::async_trait]
impl GrpcUserService for UserService {
    async fn insert_user(
        &self,
        request:Request<InsertUserRequest>
    ) -> Result<Response<User>,Status> {
        
        let payload = request.into_inner();

        Ok(Response::new(User {
            email:payload.email,
            role:payload.role,
            ..User::default()
        }))
    }
    async fn get_user(
        &self,
        request:Request<GetUserRequest>
    ) -> Result<Response<User>,Status> {
        todo!("Create user")
    }
    async fn update_user_email(
        &self,
        request:Request<UpdateUserEmailRequest>
    ) -> Result<Response<User>,Status> {
        let payload = request.into_inner();

        Ok(Response::new(User {
            email:payload.new_email,
            ..User::default()
        }))
    }
    
    async fn update_user_password(
        &self,
        request:Request<UpdateUserPasswordRequest>
    ) -> Result<Response<User>,Status> {
        let payload = request.into_inner();

        Ok(Response::new(User {
            id:payload.id,
            ..User::default()
        }))
    }
    async fn update_user_role(
        &self,
        request:Request<UpdateUserRoleRequest>
    ) -> Result<Response<User>,Status> {
        let payload = request.into_inner();

        Ok(Response::new(User {
            id:payload.id,
            role:payload.role,
            ..User::default()
        }))
    }
    async fn delete_user(
        &self,
        request:Request<DeleteUserRequest>
    ) -> Result<Response<Empty>,Status> {
        Ok(Response::new(Empty {}))
    }
}

#[cfg(test)]
mod test {

    use tonic::transport::Server;

    use crate::{models::employee_management::{user_service_client::UserServiceClient, Role}, utils::test::start_server};

    use super::*;

    #[tokio::test]
    async fn test_insert_user() {
        let (handle,channel) = start_server(
            Server::builder().add_service(UserService::new())
        ).await;

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
        assert_eq!(payload.email,"johndoe@gmail.com");
        assert_eq!(payload.role,Role::Admin as i32);
    }

    #[tokio::test]
    async fn test_update_user_email() {
        let (handle,channel) = start_server(
            Server::builder().add_service(UserService::new())
        ).await;

        let mut client = UserServiceClient::new(channel);

        let request = UpdateUserEmailRequest {
            current_email:"johndoe@gmail.com".into(),
            new_email:"janedoe@gmail.com".into(),
            id:"randomid".into(),
        };

        let response = client.update_user_email(request).await;
        
        assert!(response.is_ok());

        let response = response.unwrap();

        let payload = response.into_inner();
        assert_eq!(payload.email,"janedoe@gmail.com");
    }

    #[tokio::test]
    async fn test_update_user_password() {
        let (handle,channel) = start_server(
            Server::builder().add_service(UserService::new())
        ).await;

        let mut client = UserServiceClient::new(channel);

        let request = UpdateUserPasswordRequest {
            current_password:"johndoepassword".into(),
            new_password:"janedoepassword".into(),
            id:"randomid".into(),
        };

        let response = client.update_user_password(request).await;
        
        assert!(response.is_ok());

        let response = response.unwrap();

        let payload = response.into_inner();
        assert_eq!(payload.id,"randomid");
    }

    #[tokio::test]
    async fn test_update_user_role() {
        let (handle,channel) = start_server(
            Server::builder().add_service(UserService::new())
        ).await;

        let mut client = UserServiceClient::new(channel);

        let request = UpdateUserRoleRequest {
            id:"randomid".into(),
            role:Role::SuperAdmin as i32,
        };

        let response = client.update_user_role(request).await;
        
        assert!(response.is_ok());

        let response = response.unwrap();

        let payload = response.into_inner();
        assert_eq!(payload.id,"randomid");
        assert_eq!(payload.role,Role::SuperAdmin as i32);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let (handle,channel) = start_server(
            Server::builder().add_service(UserService::new())
        ).await;

        let mut client = UserServiceClient::new(channel);

        let request = DeleteUserRequest {
            id:"randomid".into(),
        };

        let response = client.delete_user(request).await;
        
        assert!(response.is_ok());

        let response = response.unwrap();

        let payload = response.into_inner();
        assert_eq!(payload,Empty {});
    }

}
    