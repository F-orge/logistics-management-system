use sea_orm::Set;
use tonic::Status;

use super::{
    _entities::user::{ActiveModel, Model},
    _proto::employee_management::{InsertUserRequest, Role as GrpcRole, User},
};

impl TryInto<ActiveModel> for InsertUserRequest {
    type Error = Status;

    fn try_into(self) -> Result<ActiveModel, Self::Error> {
        let role = match self.role {
            x if x == GrpcRole::SuperAdmin as i32 => "SUPER_ADMIN",
            x if x == GrpcRole::Admin as i32 => "ADMIN",
            x if x == GrpcRole::Employee as i32 => "EMPLOYEE",
            x if x == GrpcRole::Client as i32 => "CLIENT",
            _ => return Err(Status::invalid_argument("Invalid role")),
        };

        Ok(ActiveModel {
            email: Set(self.email),
            password: Set(self.password),
            user_role: Set(role.to_string()),
            ..Default::default()
        })
    }
}

impl TryInto<User> for Model {
    type Error = ();

    fn try_into(self) -> Result<User, Self::Error> {
        let role = match self.user_role {
            x if x == "SUPER_ADMIN" => GrpcRole::SuperAdmin as i32,
            x if x == "ADMIN" => GrpcRole::Admin as i32,
            x if x == "EMPLOYEE" => GrpcRole::Employee as i32,
            x if x == "CLIENT" => GrpcRole::Client as i32,
            _ => return Err(()),
        };

        Ok(User {
            id: self.id.to_string(),
            email: self.email.to_string(),
            role,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        })
    }
}
