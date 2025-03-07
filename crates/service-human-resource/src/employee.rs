use std::str::FromStr;

use hmac::Hmac;
use lib_core::error::Error;
use lib_entity::{employee, sea_orm_active_enums::{EmployeeRole, EmployeeStatus, EmployeeContractType}};
use lib_proto::management::{
    human_resource_service_server::HumanResourceService as GrpcHumanResourceService, Employee,
}; 
use sea_orm::{
    prelude::Date, ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Condition,
    DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set, TransactionTrait,
};
use sha2::Sha256;
use sqlx::{types::Uuid};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Response, Status};

pub struct HumanResourceService {
    db: DatabaseConnection,
    encryption_key: Hmac<Sha256>,
}

#[tonic::async_trait]
impl GrpcHumanResourceService for HumanResourceService {
    async fn create_employee(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        // Permission. only admin can create employee with the role of employee
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can create employee"));
        }

        let payload = request.into_inner();

        let mut auth_user_active_model = lib_entity::users::ActiveModel::new();

        auth_user_active_model.email = Set(payload.email.clone());
        auth_user_active_model.password = Set("Password".into());
        auth_user_active_model.auth_type =
            Set(lib_entity::sea_orm_active_enums::AuthType::BasicAuth);

        let auth_user = auth_user_active_model
            .insert(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        let mut active_model = lib_entity::employee::ActiveModel::new();

        active_model.auth_user_id = Set(Some(auth_user.id));

        if let Some(avatar_photo) = payload.avatar_photo {
            active_model.avatar_id = Set(Some(avatar_photo.id.parse().map_err(Error::Uuid)?));
        }

        if let Some(cover_photo) = payload.cover_photo {
            active_model.cover_photo_id = Set(Some(cover_photo.id.parse().map_err(Error::Uuid)?));
        }

        active_model.first_name = Set(payload.first_name);
        active_model.middle_name = Set(payload.middle_name);
        active_model.last_name = Set(payload.last_name);

        if let Some(tel_number) = payload.tel_number {
            active_model.tel_number = Set(Some(tel_number));
        }

        if let Some(mobile_number) = payload.mobile_number {
            active_model.mobile_number = Set(Some(mobile_number));
        }

        active_model.email = Set(Some(payload.email));
        active_model.role = Set(lib_entity::sea_orm_active_enums::EmployeeRole::Employee);
        active_model.status = Set(lib_entity::sea_orm_active_enums::EmployeeStatus::Active);
        active_model.contract_type =
            Set(lib_entity::sea_orm_active_enums::EmployeeContractType::FullTime);
        active_model.phil_nat_id = Set(payload.phil_nat_id);
        active_model.birth_date = Set(Date::from_str(&payload.birth_date)
            .map_err(|_| Status::invalid_argument("Invalid date"))?);
        active_model.special_interests = Set(Some(payload.special_interests));
        active_model.learning_institutions = Set(payload.learning_institutions);
        active_model.spouse_first_name = Set(payload.spouse_first_name);
        active_model.spouse_middle_name = Set(payload.spouse_middle_name);
        active_model.spouse_last_name = Set(payload.spouse_last_name);
        active_model.spouse_employer = Set(payload.spouse_employer);

        active_model.insert(&trx).await.map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn create_manager(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        // Permission. only admin can create employee with the role of employee
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can create employee"));
        }

        let payload = request.into_inner();

        let mut auth_user_active_model = lib_entity::users::ActiveModel::new();

        auth_user_active_model.email = Set(payload.email.clone());
        auth_user_active_model.password = Set("Password".into());
        auth_user_active_model.auth_type =
            Set(lib_entity::sea_orm_active_enums::AuthType::BasicAuth);

        let auth_user = auth_user_active_model
            .insert(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        let mut active_model = lib_entity::employee::ActiveModel::new();

        active_model.auth_user_id = Set(Some(auth_user.id));

        if let Some(avatar_photo) = payload.avatar_photo {
            active_model.avatar_id = Set(Some(avatar_photo.id.parse().map_err(Error::Uuid)?));
        }

        if let Some(cover_photo) = payload.cover_photo {
            active_model.cover_photo_id = Set(Some(cover_photo.id.parse().map_err(Error::Uuid)?));
        }

        active_model.first_name = Set(payload.first_name);
        active_model.middle_name = Set(payload.middle_name);
        active_model.last_name = Set(payload.last_name);

        if let Some(tel_number) = payload.tel_number {
            active_model.tel_number = Set(Some(tel_number));
        }

        if let Some(mobile_number) = payload.mobile_number {
            active_model.mobile_number = Set(Some(mobile_number));
        }

        active_model.email = Set(Some(payload.email));
        active_model.role = Set(lib_entity::sea_orm_active_enums::EmployeeRole::Manager);
        active_model.status = Set(lib_entity::sea_orm_active_enums::EmployeeStatus::Active);
        active_model.contract_type =
            Set(lib_entity::sea_orm_active_enums::EmployeeContractType::FullTime);
        active_model.phil_nat_id = Set(payload.phil_nat_id);
        active_model.birth_date = Set(Date::from_str(&payload.birth_date)
            .map_err(|_| Status::invalid_argument("Invalid date"))?);
        active_model.special_interests = Set(Some(payload.special_interests));
        active_model.learning_institutions = Set(payload.learning_institutions);
        active_model.spouse_first_name = Set(payload.spouse_first_name);
        active_model.spouse_middle_name = Set(payload.spouse_middle_name);
        active_model.spouse_last_name = Set(payload.spouse_last_name);
        active_model.spouse_employer = Set(payload.spouse_employer);

        active_model.insert(&trx).await.map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn create_admin(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        // Permission. only admin can create employee with the role of employee
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can create employee"));
        }

        let payload = request.into_inner();

        let mut auth_user_active_model = lib_entity::users::ActiveModel::new();

        auth_user_active_model.email = Set(payload.email.clone());
        auth_user_active_model.password = Set("Password".into());
        auth_user_active_model.auth_type =
            Set(lib_entity::sea_orm_active_enums::AuthType::BasicAuth);

        let auth_user = auth_user_active_model
            .insert(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        let mut active_model = lib_entity::employee::ActiveModel::new();

        active_model.auth_user_id = Set(Some(auth_user.id));

        if let Some(avatar_photo) = payload.avatar_photo {
            active_model.avatar_id = Set(Some(avatar_photo.id.parse().map_err(Error::Uuid)?));
        }

        if let Some(cover_photo) = payload.cover_photo {
            active_model.cover_photo_id = Set(Some(cover_photo.id.parse().map_err(Error::Uuid)?));
        }

        active_model.first_name = Set(payload.first_name);
        active_model.middle_name = Set(payload.middle_name);
        active_model.last_name = Set(payload.last_name);

        if let Some(tel_number) = payload.tel_number {
            active_model.tel_number = Set(Some(tel_number));
        }

        if let Some(mobile_number) = payload.mobile_number {
            active_model.mobile_number = Set(Some(mobile_number));
        }

        active_model.email = Set(Some(payload.email));
        active_model.role = Set(lib_entity::sea_orm_active_enums::EmployeeRole::Admin);
        active_model.status = Set(lib_entity::sea_orm_active_enums::EmployeeStatus::Active);
        active_model.contract_type =
            Set(lib_entity::sea_orm_active_enums::EmployeeContractType::FullTime);
        active_model.phil_nat_id = Set(payload.phil_nat_id);
        active_model.birth_date = Set(Date::from_str(&payload.birth_date)
            .map_err(|_| Status::invalid_argument("Invalid date"))?);
        active_model.special_interests = Set(Some(payload.special_interests));
        active_model.learning_institutions = Set(payload.learning_institutions);
        active_model.spouse_first_name = Set(payload.spouse_first_name);
        active_model.spouse_middle_name = Set(payload.spouse_middle_name);
        active_model.spouse_last_name = Set(payload.spouse_last_name);
        active_model.spouse_employer = Set(payload.spouse_employer);

        active_model.insert(&trx).await.map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn add_special_interest(
        &self,
        request: tonic::Request<lib_proto::management::AddSpecialInterestRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        // Permission: Admin, Employee and Manager can add special interest. employee can only add special interest to themselves

        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let employee = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        let payload = request.into_inner();

        if employee.role == lib_entity::sea_orm_active_enums::EmployeeRole::Employee
            && employee.id != claims.subject
        {
            return Err(Status::permission_denied(
                "Employee can only add special interest to themselves",
            ));
        }

        let mut new_special_interest = employee.special_interests.clone().unwrap_or_default();

        new_special_interest.push(payload.special_interest);

        let mut employee_active_model = employee.into_active_model();

        employee_active_model.special_interests = Set(Some(new_special_interest));

        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn add_learning_institution(
        &self,
        request: tonic::Request<lib_proto::management::AddLearningInstitutionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let employee = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        let payload = request.into_inner();

        if employee.role == lib_entity::sea_orm_active_enums::EmployeeRole::Employee
            && employee.id != claims.subject
        {
            return Err(Status::permission_denied(
                "Employee can only add special interest to themselves",
            ));
        }

        let mut new_learning_institutions = employee.learning_institutions.clone();

        new_learning_institutions.push(payload.learning_institution);

        let mut employee_active_model = employee.into_active_model();

        employee_active_model.learning_institutions = Set(new_learning_institutions);

        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn get_employee(
        &self,
        request: tonic::Request<lib_proto::management::GetEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::Employee>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        // Permission: only registered employees can access employee information
        _ = lib_entity::employee::Entity::find()
            .filter(lib_entity::employee::Column::AuthUserId.eq(claims.subject))
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        let payload = request.into_inner();

        let employee = lib_entity::employee::Entity::find_by_id(
            payload.id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&self.db)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;

        Ok(Response::new(employee.into()))
    }

    type GetEmployeesByDepartmentStream = ReceiverStream<Result<Employee, Status>>;

    async fn get_employees_by_department(
        &self,
        request: tonic::Request<lib_proto::management::GetEmployeesByDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<Self::GetEmployeesByDepartmentStream>, tonic::Status>
    {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        // Permission: only registered employees can access employee information
        _ = lib_entity::employee::Entity::find()
            .filter(lib_entity::employee::Column::AuthUserId.eq(claims.subject))
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        let payload = request.into_inner();

        let mut employee = lib_entity::employee::Entity::find()
            .inner_join(lib_entity::department_employees::Entity)
            .filter(
                lib_entity::department_employees::Column::DepartmentId
                    .eq(payload.department_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .stream(&self.db)
            .await
            .map_err(Error::SeaOrm)?;

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<Employee, Status>>(1024);

        while let Some(row) = employee.try_next().await.map_err(Error::SeaOrm)? {
            let item: employee::Model = row.into();
            tx.send(Ok(item.into()))
                .await
                .map_err(|err| Error::Custom(Box::new(err)))?;
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn change_employee_avatar(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeAvatarRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let payload = request.into_inner();

        // Permission: Only admin or employee can update employee avatar. only employee can update their own avatar
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();

        if let Some(avatar_photo) = payload.avatar_photo {
            employee_active_model.avatar_id =
                Set(Some(avatar_photo.id.parse().map_err(Error::Uuid)?));
        }

        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn change_employee_cover_photo(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeCoverPhotoRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let payload = request.into_inner();

        // Permission: Only admin or employee can update employee avatar. only employee can update their own avatar
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();

        if let Some(cover_photo) = payload.cover_photo {
            employee_active_model.cover_photo_id =
                Set(Some(cover_photo.id.parse().map_err(Error::Uuid)?));
        }

        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn change_employee_first_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeFirstNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let payload = request.into_inner();

        // Permission: Only admin or employee can update employee avatar. only employee can update their own avatar
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();

        employee_active_model.first_name = Set(payload.first_name);

        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn change_employee_middle_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeMiddleNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update employee middle name. Only employee can update their own middle name.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.middle_name = Set(payload.middle_name);
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_employee_last_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeLastNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update employee last name. Only employee can update their own last name.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.last_name = Set(payload.last_name);
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_employee_tel_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeTelNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update employee telephone number. Only employee can update their own telephone number.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.tel_number = Set(Some(payload.tel_number));
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_employee_mobile_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeMobileNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update employee mobile number. Only employee can update their own mobile number.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.mobile_number = Set(Some(payload.mobile_number));
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_employee_email(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeEmailRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update employee email. Only employee can update their own email.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.email = Set(Some(payload.email));
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_employee_role(
    &self,
    request: tonic::Request<lib_proto::management::ChangeEmployeeRoleRequest>,
) -> std::result::Result<tonic::Response<()>, tonic::Status> {
    let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

    let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

    // Permission: Only admin can update employee roles.
    let role = lib_entity::employee::Entity::find_by_id(claims.subject)
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;

    // Fix: Compare with the enum variant, not an integer.
    if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
        return Err(Status::permission_denied("Only admin can update employee roles"));
    }

    let payload = request.into_inner();

    let mut employee_active_model = lib_entity::employee::Entity::find_by_id(
        payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?,
    )
    .one(&trx)
    .await
    .map_err(Error::SeaOrm)?
    .ok_or(Error::RowNotFound)?
    .into_active_model();

    // Fix: Assign the enum variant, not an integer.
    let new_role = match payload.role {
        0 => EmployeeRole::Admin,
        1 => EmployeeRole::Manager,
        2 => EmployeeRole::Employee,
        _ => return Err(Status::invalid_argument("Invalid role value")),
    };
    
    employee_active_model.role = Set(new_role);

    employee_active_model
        .update(&trx)
        .await
        .map_err(Error::SeaOrm)?;

    trx.commit().await.map_err(Error::SeaOrm)?;

    Ok(Response::new(()))
}

    async fn change_employee_status(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeStatusRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin can update employee status.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can change employee status"));
        }
    
        let mut employee_active_model = lib_entity::employee::Entity::find_by_id(
            payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        let new_status = match payload.status {
            0 => EmployeeStatus::Active,
            1 => EmployeeStatus::Inactive,
            _ => return Err(Status::invalid_argument("Invalid status value")),
        };
        
        employee_active_model.status = Set(new_status);
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_employee_contract_type(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeContractTypeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin can update employee contract type.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can change employee contract type"));
        }
    
        let mut employee_active_model = lib_entity::employee::Entity::find_by_id(
            payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        let new_contract_type = match payload.contract_type {
            0 => EmployeeContractType::FullTime,
            1 => EmployeeContractType::PartTime,
            _ => return Err(Status::invalid_argument("Invalid contract type value")),
        };
        
        employee_active_model.contract_type = Set(new_contract_type);
        
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_phil_nat_id(
        &self,
        request: tonic::Request<lib_proto::management::ChangePhilNatIdRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update employee PhilNat ID. Only employee can update their own PhilNat ID.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.phil_nat_id = Set(payload.phil_nat_id);
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_birth_date(
        &self,
        request: tonic::Request<lib_proto::management::ChangeBirthDateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update employee birth date. Only employee can update their own birth date.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.phil_nat_id = Set(payload.birth_date);
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_spouse_first_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseFirstNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update spouse's first name. Only employee can update their own spouse's first name.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.spouse_first_name = Set(Some(payload.spouse_first_name));
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_spouse_middle_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseMiddleNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update spouse's middle name. Only employee can update their own spouse's middle name.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.spouse_middle_name = Set(Some(payload.spouse_middle_name));
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_spouse_last_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseLastNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update spouse's last name. Only employee can update their own spouse's last name.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.spouse_last_name = Set(Some(payload.spouse_last_name));
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_spouse_employer(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseEmployerRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can update spouse's employer. Only employee can update their own spouse's employer.
        let mut employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();
    
        employee_active_model.spouse_employer = Set(Some(payload.spouse_employer));
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn remove_employee(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let payload = request.into_inner();

        // Permission: Only admin or employee can update employee avatar. only employee can update their own avatar
        let employee_active_model = lib_entity::employee::Entity::find()
            .filter(
                lib_entity::employee::Column::AuthUserId
                    .eq(payload.id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                Condition::any()
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Employee)
                            .and(lib_entity::employee::Column::Id.eq(claims.subject)),
                    )
                    .add(
                        lib_entity::employee::Column::Role
                            .eq(lib_entity::sea_orm_active_enums::EmployeeRole::Admin),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();

        employee_active_model
            .delete(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn remove_special_interest(
        &self,
        request: tonic::Request<lib_proto::management::RemoveSpecialInterestRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        let payload = request.into_inner();
    
        // Permission: Only admin or employee can remove special interests. Only employee can remove their own special interests.
        let employee = lib_entity::employee::Entity::find_by_id(
            payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;
    
        if employee.role == lib_entity::sea_orm_active_enums::EmployeeRole::Employee
            && employee.id != claims.subject
        {
            return Err(Status::permission_denied(
                "Employee can only remove special interests from themselves",
            ));
        }
    
        let mut special_interests = employee.special_interests.clone().unwrap_or_default();
    
        // Remove the specified special interest
        special_interests.retain(|interest| interest != &payload.special_interest);
    
        let mut employee_active_model = employee.into_active_model();
        employee_active_model.special_interests = Set(Some(special_interests));
    
        employee_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn remove_learning_institition(
        &self,
        request: tonic::Request<lib_proto::management::RemoveLearningInstitutionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

    let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

    let payload = request.into_inner();

    // Permission: Only admin or employee can remove learning institutions. Only employee can remove their own learning institutions.
    let employee = lib_entity::employee::Entity::find_by_id(
        payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?,
    )
    .one(&trx)
    .await
    .map_err(Error::SeaOrm)?
    .ok_or(Error::RowNotFound)?;

    if employee.role == lib_entity::sea_orm_active_enums::EmployeeRole::Employee
        && employee.id != claims.subject
    {
        return Err(Status::permission_denied(
            "Employee can only remove learning institutions from themselves",
        ));
    }

    let mut learning_institutions = employee.learning_institutions.clone();

    // Remove the specified learning institution
    learning_institutions.retain(|institution| institution != &payload.learning_institution);

    let mut employee_active_model = employee.into_active_model();
    employee_active_model.learning_institutions = Set(learning_institutions);

    employee_active_model
        .update(&trx)
        .await
        .map_err(Error::SeaOrm)?;

    trx.commit().await.map_err(Error::SeaOrm)?;

    Ok(Response::new(()))
    }

    async fn create_department(
        &self,
        request: tonic::Request<lib_proto::management::CreateDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can create departments.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can create departments"));
        }
    
        let payload = request.into_inner();
    
        let mut department_active_model = lib_entity::department::ActiveModel::new();
        department_active_model.name = Set(payload.name);
        department_active_model.description = Set(payload.description);
    
        department_active_model
            .insert(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn add_employee_to_department(
        &self,
        request: tonic::Request<lib_proto::management::AddEmployeeToDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can add employees to departments.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can add employees to departments"));
        }
    
        let payload = request.into_inner();
    
        let mut department_employee_active_model = lib_entity::department_employees::ActiveModel::new();
        department_employee_active_model.department_id = Set(payload.department_id.parse::<Uuid>().map_err(Error::Uuid)?);
        department_employee_active_model.employee_id = Set(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?);
    
        department_employee_active_model
            .insert(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn get_department(
        &self,
        request: tonic::Request<lib_proto::management::GetDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::Department>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        // Permission: Only registered employees can access department information.
        _ = lib_entity::employee::Entity::find()
            .filter(lib_entity::employee::Column::AuthUserId.eq(claims.subject))
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        let payload = request.into_inner();
    
        let department = lib_entity::department::Entity::find_by_id(
            payload.id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&self.db)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;
    
        Ok(Response::new(department.into()))
    }

    async fn update_department_name(
        &self,
        request: tonic::Request<lib_proto::management::UpdateDepartmentNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can update department names.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update department names"));
        }
    
        let payload = request.into_inner();
    
        let mut department_active_model = lib_entity::department::Entity::find_by_id(
            payload.name.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        department_active_model.name = Set(payload.name);
    
        department_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn update_department_description(
        &self,
        request: tonic::Request<lib_proto::management::UpdateDepartmentDescriptionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can update department descriptions.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update department descriptions"));
        }
    
        let payload = request.into_inner();
    
        let mut department_active_model = lib_entity::department::Entity::find_by_id(
            payload.description.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        department_active_model.description = Set(Some(payload.description));
    
        department_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn remove_department(
        &self,
        request: tonic::Request<lib_proto::management::RemoveDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can remove departments.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can remove departments"));
        }
    
        let payload = request.into_inner();
    
        let department = lib_entity::department::Entity::find_by_id(
            payload.id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;
    
        department
            .into_active_model()
            .delete(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn remove_employee_to_department(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmployeeFromDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can remove employees from departments.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can remove employees from departments"));
        }
    
        let payload = request.into_inner();
    
        let department_employee = lib_entity::department_employees::Entity::find()
            .filter(
                lib_entity::department_employees::Column::DepartmentId
                    .eq(payload.department_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .filter(
                lib_entity::department_employees::Column::EmployeeId
                    .eq(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?),
            )
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        department_employee
            .into_active_model()
            .delete(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn create_job_information(
        &self,
        request: tonic::Request<lib_proto::management::CreateJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can create job information.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can create job information"));
        }
    
        let payload = request.into_inner();
    
        let mut job_information_active_model = lib_entity::job_information::ActiveModel::new();
        job_information_active_model.employee_id = Set(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?);
        job_information_active_model.title = Set(payload.title);
        job_information_active_model.department_id = Set(payload.department_id.parse::<Uuid>().map_err(Error::Uuid)?);
        job_information_active_model.supervisor_id = Set(payload.supervisor_id.parse::<Uuid>().map_err(Error::Uuid)?);
        job_information_active_model.work_location = Set(payload.work_location);
        job_information_active_model.start_date = Set(Date::from_str(&payload.start_date).map_err(|_| Status::invalid_argument("Invalid date"))?);
        job_information_active_model.salary = Set(payload.salary.parse().map_err(|_| Status::invalid_argument("Invalid salary"))?);
        job_information_active_model.currency = Set(payload.currency);
    
        job_information_active_model
            .insert(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn get_job_information(
        &self,
        request: tonic::Request<lib_proto::management::GetJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::JobInformation>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        // Permission: Only registered employees can access job information.
        _ = lib_entity::employee::Entity::find()
            .filter(lib_entity::employee::Column::AuthUserId.eq(claims.subject))
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        let payload = request.into_inner();
    
        let job_information = lib_entity::job_information::Entity::find_by_id(
            payload.id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&self.db)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;
    
        Ok(Response::new(job_information.into()))
    }

    async fn change_job_title(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobTitleRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        // Permission: Only admin can update job titles.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update job titles"));
        }

        let payload = request.into_inner();

        let mut job_information_active_model = lib_entity::job_information::Entity::find_by_id(
            payload.title.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();

        job_information_active_model.title = Set(payload.title);

        job_information_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn change_job_department(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can update job departments.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update job departments"));
        }
    
        let payload = request.into_inner();
    
        let mut job_information_active_model = lib_entity::job_information::Entity::find_by_id(
            payload.department_id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        job_information_active_model.department_id = Set(payload.department_id.parse::<Uuid>().map_err(Error::Uuid)?);
    
        job_information_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_job_supervisor(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobSupervisorRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can update job supervisors.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update job supervisors"));
        }
    
        let payload = request.into_inner();
    
        let mut job_information_active_model = lib_entity::job_information::Entity::find_by_id(
            payload.supervisor_id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        job_information_active_model.supervisor_id = Set(payload.supervisor_id.parse::<Uuid>().map_err(Error::Uuid)?);
    
        job_information_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_job_work_location(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobWorkLocationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can update job work locations.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update job work locations"));
        }
    
        let payload = request.into_inner();
    
        let mut job_information_active_model = lib_entity::job_information::Entity::find_by_id(
            payload.work_location.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        job_information_active_model.work_location = Set(payload.work_location);
    
        job_information_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_job_start_date(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobStartDateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can update job start dates.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update job start dates"));
        }
    
        let payload = request.into_inner();
    
        let mut job_information_active_model = lib_entity::job_information::Entity::find_by_id(
            payload.start_date.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        job_information_active_model.start_date = Set(Date::from_str(&payload.start_date).map_err(|_| Status::invalid_argument("Invalid date"))?);
    
        job_information_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_job_salary(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobSalaryRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can update job salaries.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update job salaries"));
        }
    
        let payload = request.into_inner();
    
        let mut job_information_active_model = lib_entity::job_information::Entity::find_by_id(
            payload.salary.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        job_information_active_model.salary = Set(payload.salary.parse().map_err(|_| Status::invalid_argument("Invalid salary"))?);
    
        job_information_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn change_job_currency(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobCurrencyRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can update job currencies.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can update job currencies"));
        }
    
        let payload = request.into_inner();
    
        let mut job_information_active_model = lib_entity::job_information::Entity::find_by_id(
            payload.currency.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();
    
        job_information_active_model.currency = Set(payload.currency);
    
        job_information_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn remove_job_information(
        &self,
        request: tonic::Request<lib_proto::management::RemoveJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can remove job information.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can remove job information"));
        }
    
        let payload = request.into_inner();
    
        let job_information = lib_entity::job_information::Entity::find_by_id(
            payload.id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;
    
        job_information
            .into_active_model()
            .delete(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn create_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmergencyInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
    
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
    
        // Permission: Only admin can create emergency information.
        let role = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;
    
        if role.role != lib_entity::sea_orm_active_enums::EmployeeRole::Admin {
            return Err(Status::permission_denied("Only admin can create emergency information"));
        }
    
        let payload = request.into_inner();
    
        let mut emergency_information_active_model = lib_entity::emergency_information::ActiveModel::new();
        emergency_information_active_model.employee_id = Set(payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?);
        emergency_information_active_model.address = Set(Some(payload.address));
        emergency_information_active_model.tel_number = Set(payload.tel_number);
        emergency_information_active_model.mobile_number = Set(payload.mobile_number);
        emergency_information_active_model.contact_name = Set(payload.contact_name);
    
        emergency_information_active_model
            .insert(&trx)
            .await
            .map_err(Error::SeaOrm)?;
    
        trx.commit().await.map_err(Error::SeaOrm)?;
    
        Ok(Response::new(()))
    }

    async fn add_emergency_information_health_condition(
        &self,
        request: tonic::Request<
            lib_proto::management::AddEmergencyInformationHealthConditionRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let employee = lib_entity::employee::Entity::find_by_id(claims.subject)
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        let payload = request.into_inner();

        if employee.role == lib_entity::sea_orm_active_enums::EmployeeRole::Employee
            && employee.id != claims.subject
        {
            return Err(Status::permission_denied(
            "Employee can only add health conditions to their own emergency information",
            ));
        }

        let mut emergency_info = lib_entity::emergency_information::Entity::find_by_id(
            payload.health_condition.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;

        let mut health_conditions = emergency_info.health_conditions.clone().unwrap_or_default();

        health_conditions.push(payload.health_condition);

        let mut emergency_info_active_model = emergency_info.into_active_model();
        emergency_info_active_model.health_conditions = Set(Some(health_conditions));

        emergency_info_active_model
            .update(&trx)
            .await
            .map_err(Error::SeaOrm)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn get_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::GetEmergencyInformationRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::management::EmployeeEmergencyInformation>,
        tonic::Status,
    > {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;

        // Permission: Only registered employees can access emergency information.
        _ = lib_entity::employee::Entity::find()
            .filter(lib_entity::employee::Column::AuthUserId.eq(claims.subject))
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        let payload = request.into_inner();

        let emergency_info = lib_entity::emergency_information::Entity::find_by_id(
            payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&self.db)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;

        Ok(Response::new(emergency_info.into()))
    }

    async fn change_emergency_information_address(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmergencyInformationAddressRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
        let payload = request.into_inner();

        let mut emergency_info_active_model = lib_entity::emergency_information::Entity::find_by_id(
            payload.address.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();

        emergency_info_active_model.address = Set(Some(payload.address));

        emergency_info_active_model.update(&trx).await.map_err(Error::SeaOrm)?;
        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn change_emergency_information_tel_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmergencyInformationTelNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
        let payload = request.into_inner();

        let mut emergency_info_active_model = lib_entity::emergency_information::Entity::find_by_id(
            payload.tel_number.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();

        emergency_info_active_model.tel_number = Set(Some(payload.tel_number));

        emergency_info_active_model.update(&trx).await.map_err(Error::SeaOrm)?;
        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn change_emergency_information_mobile_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmergencyInformationMobileNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
        let payload = request.into_inner();

        let mut emergency_info_active_model = lib_entity::emergency_information::Entity::find_by_id(
            payload.mobile_number.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();

        emergency_info_active_model.mobile_number = Set(Some(payload.mobile_number));

        emergency_info_active_model.update(&trx).await.map_err(Error::SeaOrm)?;
        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn change_emergency_information_contact_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmergencyInformationContactNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
        let payload = request.into_inner();

        let mut emergency_info_active_model = lib_entity::emergency_information::Entity::find_by_id(
            payload.contact_name.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();

        emergency_info_active_model.contact_name = Set(payload.contact_name);

        emergency_info_active_model.update(&trx).await.map_err(Error::SeaOrm)?;
        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn remove_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmergencyInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
        let payload = request.into_inner();

        let emergency_info_active_model = lib_entity::emergency_information::Entity::find_by_id(
            payload.employee_id.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?
        .into_active_model();

        emergency_info_active_model.delete(&trx).await.map_err(Error::SeaOrm)?;
        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn remove_emergency_information_health_condition(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmergencyInformationHealthConditionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let claims = lib_security::get_jwt_claim(request.metadata(), &self.encryption_key)?;
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;
        let payload = request.into_inner();

        let mut emergency_info = lib_entity::emergency_information::Entity::find_by_id(
            payload.health_condition.parse::<Uuid>().map_err(Error::Uuid)?,
        )
        .one(&trx)
        .await
        .map_err(Error::SeaOrm)?
        .ok_or(Error::RowNotFound)?;

        let mut health_conditions = emergency_info.health_conditions.clone().unwrap_or_default();
        health_conditions.retain(|condition| condition != &payload.health_condition);

        let mut emergency_info_active_model = emergency_info.into_active_model();
        emergency_info_active_model.health_conditions = Set(Some(health_conditions));

        emergency_info_active_model.update(&trx).await.map_err(Error::SeaOrm)?;
        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn create_pan_employee_request(
        &self,
        request: tonic::Request<lib_proto::management::CreatePanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn get_pan_information(
        &self,
        request: tonic::Request<lib_proto::management::GetPanInformationRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::PersonnelAction>, tonic::Status>
    {
        todo!()
    }

    async fn change_pan_action_type(
        &self,
        request: tonic::Request<lib_proto::management::ChangePanActionTypeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_pan_old_value(
        &self,
        request: tonic::Request<lib_proto::management::ChangePanOldValueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_pan_new_value(
        &self,
        request: tonic::Request<lib_proto::management::ChangePanNewValueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_pan_effective_date(
        &self,
        request: tonic::Request<lib_proto::management::ChangePanEffectiveDateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn approve_pan(
        &self,
        request: tonic::Request<lib_proto::management::ApprovePanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn reject_pan(
        &self,
        request: tonic::Request<lib_proto::management::RejectPanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn remove_pan_information(
        &self,
        request: tonic::Request<lib_proto::management::RemovePanInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}

