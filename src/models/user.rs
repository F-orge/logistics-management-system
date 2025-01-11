use sea_orm::Set;
use tonic::Status;

use super::{
    _entities::{
        sea_orm_active_enums::RoleEnum,
        user::{ActiveModel, Model},
    },
    _proto::employee_management::{InsertUserRequest, Role, User},
};

impl TryInto<ActiveModel> for InsertUserRequest {
    type Error = Status;

    fn try_into(self) -> Result<ActiveModel, Self::Error> {
        let role: RoleEnum = match self.role {
            x if x == Role::SuperAdmin as i32 => RoleEnum::SuperAdmin,
            x if x == Role::Admin as i32 => RoleEnum::Admin,
            x if x == Role::Employee as i32 => RoleEnum::Employee,
            x if x == Role::Client as i32 => RoleEnum::Client,
            _ => return Err(Status::invalid_argument("Invalid role")),
        };

        Ok(ActiveModel {
            email: Set(self.email),
            password: Set(self.password),
            role: Set(role),
            ..Default::default()
        })
    }
}

impl TryInto<User> for Model {
    type Error = ();

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User {
            id: self.id.to_string(),
            email: self.email.to_string(),
            role: self.role as i32,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
        })
    }
}
