use std::str::FromStr;

use hmac::Hmac;
use lib_core::error::Error;
use lib_entity::employee;
use lib_proto::management::{
    human_resource_service_server::HumanResourceService as GrpcHumanResourceService, Employee,
};
use sea_orm::{
    prelude::Date, ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Condition,
    DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set, TransactionTrait,
};
use sha2::Sha256;
use sqlx::{types::Uuid, Pool, Postgres};
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
        todo!()
    }

    async fn change_employee_last_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeLastNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_employee_tel_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeTelNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_employee_mobile_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeMobileNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_employee_email(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeEmailRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_employee_role(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeRoleRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_employee_status(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeStatusRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_employee_contract_type(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeContractTypeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_phil_nat_id(
        &self,
        request: tonic::Request<lib_proto::management::ChangePhilNatIdRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_birth_date(
        &self,
        request: tonic::Request<lib_proto::management::ChangeBirthDateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_spouse_first_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseFirstNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_spouse_middle_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseMiddleNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_spouse_last_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseLastNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_spouse_employer(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseEmployerRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
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
        todo!()
    }

    async fn remove_learning_institition(
        &self,
        request: tonic::Request<lib_proto::management::RemoveLearningInstitutionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn create_department(
        &self,
        request: tonic::Request<lib_proto::management::CreateDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn add_employee_to_department(
        &self,
        request: tonic::Request<lib_proto::management::AddEmployeeToDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn get_department(
        &self,
        request: tonic::Request<lib_proto::management::GetDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::Department>, tonic::Status>
    {
        todo!()
    }

    async fn update_department_name(
        &self,
        request: tonic::Request<lib_proto::management::UpdateDepartmentNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn update_department_description(
        &self,
        request: tonic::Request<lib_proto::management::UpdateDepartmentDescriptionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn remove_department(
        &self,
        request: tonic::Request<lib_proto::management::RemoveDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn remove_employee_to_department(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmployeeFromDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn create_job_information(
        &self,
        request: tonic::Request<lib_proto::management::CreateJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn get_job_information(
        &self,
        request: tonic::Request<lib_proto::management::GetJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::JobInformation>, tonic::Status>
    {
        todo!()
    }

    async fn change_job_title(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobTitleRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_job_department(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_job_supervisor(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobSupervisorRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_job_work_location(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobWorkLocationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_job_start_date(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobStartDateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_job_salary(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobSalaryRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_job_currency(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobCurrencyRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn remove_job_information(
        &self,
        request: tonic::Request<lib_proto::management::RemoveJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn create_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmergencyInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn add_emergency_information_health_condition(
        &self,
        request: tonic::Request<
            lib_proto::management::AddEmergencyInformationHealthConditionRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn get_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::GetEmergencyInformationRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::management::EmployeeEmergencyInformation>,
        tonic::Status,
    > {
        todo!()
    }

    async fn change_emergency_information_address(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmergencyInformationAddressRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_emergency_information_tel_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmergencyInformationTelNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_emergency_information_mobile_number(
        &self,
        request: tonic::Request<
            lib_proto::management::ChangeEmergencyInformationMobileNumberRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_emergency_information_contact_name(
        &self,
        request: tonic::Request<
            lib_proto::management::ChangeEmergencyInformationContactNameRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn remove_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmergencyInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn remove_emergency_information_health_condition(
        &self,
        request: tonic::Request<
            lib_proto::management::RemoveEmergencyInformationHealthConditionRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
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

/*
#[tonic::async_trait]
impl GrpcHumanResourceService for HumanResourceService {
    async fn create_employee(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;

        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::insert()
            .into_table((Alias::new("management"), Alias::new("employee")))
            .columns([
                Alias::new("avatar_id"),
                Alias::new("cover_photo_id"),
                Alias::new("first_name"),
                Alias::new("middle_name"),
                Alias::new("last_name"),
                Alias::new("tel_number"),
                Alias::new("mobile_number"),
                Alias::new("email"),
                Alias::new("role"),
                Alias::new("status"),
                Alias::new("contract_type"),
                Alias::new("phil_nat_id"),
                Alias::new("birth_date"),
                Alias::new("special_interests"),
                Alias::new("learning_institutions"),
                Alias::new("spouse_first_name"),
                Alias::new("spouse_middle_name"),
                Alias::new("spouse_last_name"),
                Alias::new("spouse_employer"),
            ])
            .values([
                payload.avatar_photo.unwrap_or_default().id.into(),
                payload.cover_photo.unwrap_or_default().id.into(),
                payload.first_name.into(),
                payload.middle_name.into(),
                payload.last_name.into(),
                payload.tel_number.unwrap_or_default().into(),
                payload.mobile_number.unwrap_or_default().into(),
                payload.email.into(),
                "Employee".into(),
                "Active".into(),
                payload.phil_nat_id.into(),
                payload.birth_date.into(),
                payload.special_interests.into(),
                payload.learning_institutions.into(),
                payload.spouse_first_name.into(),
                payload.spouse_middle_name.into(),
                payload.spouse_last_name.into(),
                payload.spouse_employer.into(),
            ])
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn create_manager(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::insert()
            .into_table((Alias::new("management"), Alias::new("employee")))
            .columns([
                Alias::new("avatar_id"),
                Alias::new("cover_photo_id"),
                Alias::new("first_name"),
                Alias::new("middle_name"),
                Alias::new("last_name"),
                Alias::new("tel_number"),
                Alias::new("mobile_number"),
                Alias::new("email"),
                Alias::new("role"),
                Alias::new("status"),
                Alias::new("contract_type"),
                Alias::new("phil_nat_id"),
                Alias::new("birth_date"),
                Alias::new("special_interests"),
                Alias::new("learning_institutions"),
                Alias::new("spouse_first_name"),
                Alias::new("spouse_middle_name"),
                Alias::new("spouse_last_name"),
                Alias::new("spouse_employer"),
            ])
            .values([
                payload.avatar_photo.unwrap_or_default().id.into(),
                payload.cover_photo.unwrap_or_default().id.into(),
                payload.first_name.into(),
                payload.middle_name.into(),
                payload.last_name.into(),
                payload.tel_number.unwrap_or_default().into(),
                payload.mobile_number.unwrap_or_default().into(),
                payload.email.into(),
                "Manager".into(),
                "Active".into(),
                payload.phil_nat_id.into(),
                payload.birth_date.into(),
                payload.special_interests.into(),
                payload.learning_institutions.into(),
                payload.spouse_first_name.into(),
                payload.spouse_middle_name.into(),
                payload.spouse_last_name.into(),
                payload.spouse_employer.into(),
            ])
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn create_admin(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::insert()
            .into_table((Alias::new("management"), Alias::new("employee")))
            .columns([
                Alias::new("avatar_id"),
                Alias::new("cover_photo_id"),
                Alias::new("first_name"),
                Alias::new("middle_name"),
                Alias::new("last_name"),
                Alias::new("tel_number"),
                Alias::new("mobile_number"),
                Alias::new("email"),
                Alias::new("role"),
                Alias::new("status"),
                Alias::new("contract_type"),
                Alias::new("phil_nat_id"),
                Alias::new("birth_date"),
                Alias::new("special_interests"),
                Alias::new("learning_institutions"),
                Alias::new("spouse_first_name"),
                Alias::new("spouse_middle_name"),
                Alias::new("spouse_last_name"),
                Alias::new("spouse_employer"),
            ])
            .values([
                payload.avatar_photo.unwrap_or_default().id.into(),
                payload.cover_photo.unwrap_or_default().id.into(),
                payload.first_name.into(),
                payload.middle_name.into(),
                payload.last_name.into(),
                payload.tel_number.unwrap_or_default().into(),
                payload.mobile_number.unwrap_or_default().into(),
                payload.email.into(),
                "SuperAdmin".into(),
                "Active".into(),
                payload.phil_nat_id.into(),
                payload.birth_date.into(),
                payload.special_interests.into(),
                payload.learning_institutions.into(),
                payload.spouse_first_name.into(),
                payload.spouse_middle_name.into(),
                payload.spouse_last_name.into(),
                payload.spouse_employer.into(),
            ])
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn add_special_interest(
        &self,
        request: tonic::Request<lib_proto::management::AddSpecialInterestRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("special_interests"),
                format!("array_append(special_interests, '{}')", payload.special_interest),
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn add_learning_institution(
        &self,
        request: tonic::Request<lib_proto::management::AddLearningInstitutionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("learning_institutions"),
                format!("array_append(learning_institutions, '{}')", payload.learning_institution),
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn get_employee(
        &self,
        request: tonic::Request<lib_proto::management::GetEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::Employee>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;

        let (query, values) = Query::select()
            .columns([
                Alias::new("id"),
                Alias::new("avatar_id"),
                Alias::new("cover_photo_id"),
                Alias::new("first_name"),
                Alias::new("middle_name"),
                Alias::new("last_name"),
                Alias::new("tel_number"),
                Alias::new("mobile_number"),
                Alias::new("email"),
                Alias::new("role"),
                Alias::new("status"),
                Alias::new("contract_type"),
                Alias::new("phil_nat_id"),
                Alias::new("birth_date"),
                Alias::new("special_interests"),
                Alias::new("learning_institutions"),
                Alias::new("auth_user_id"),
                Alias::new("spouse_first_name"),
                Alias::new("spouse_middle_name"),
                Alias::new("spouse_last_name"),
                Alias::new("spouse_employer"),
            ])
            .from((Alias::new("management"), Alias::new("employee")))
            .and_where(Alias::new("id").eq(payload.id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_as::<_, lib_proto::management::Employee>(&query)
            .bind(&values)
            .fetch_one(&mut conn)
            .await
            .map_err(lib_core::error::Error::Database)?;

        return Ok(Response::new(result));
    }

    type GetEmployeesByDepartmentStream = ReceiverStream<Result<Employee, Status>>;

    async fn get_employees_by_department(
        &self,
        request: tonic::Request<lib_proto::management::GetEmployeesByDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<Self::GetEmployeesByDepartmentStream>, tonic::Status>
    {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;

        let (query, values) = Query::select()
            .column(Alias::new("employee_ids"))
            .from((Alias::new("management"), Alias::new("department")))
            .and_where(Alias::new("id").eq(payload.department_id.clone()))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        let department = sqlx::query_as::<_, lib_proto::management::Department>(&query)
            .bind(&values)
            .fetch_one(&mut conn)
            .await
            .map_err(lib_core::error::Error::Database)?;

        let (tx, rx) = tokio::sync::mpsc::channel(10);

        let db = self.db.clone();
        let employee_ids = department.employee_ids;

        tokio::spawn(async move {
            for employee_id in employee_ids {
                let mut conn = match lib_core::database::aquire_connection(&db).await {
                    Ok(conn) => conn,
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                };

                let (query, values) = match Query::select()
                    .columns([
                        Alias::new("id"),
                        Alias::new("avatar_id"),
                        Alias::new("cover_photo_id"),
                        Alias::new("first_name"),
                        Alias::new("middle_name"),
                        Alias::new("last_name"),
                        Alias::new("tel_number"),
                        Alias::new("mobile_number"),
                        Alias::new("email"),
                        Alias::new("role"),
                        Alias::new("status"),
                        Alias::new("contract_type"),
                        Alias::new("phil_nat_id"),
                        Alias::new("birth_date"),
                        Alias::new("special_interests"),
                        Alias::new("learning_institutions"),
                        Alias::new("auth_user_id"),
                        Alias::new("spouse_first_name"),
                        Alias::new("spouse_middle_name"),
                        Alias::new("spouse_last_name"),
                        Alias::new("spouse_employer"),
                    ])
                    .from((Alias::new("management"), Alias::new("employee")))
                    .and_where(Alias::new("id").eq(employee_id))
                    .build_sqlx(PostgresQueryBuilder) {
                    Ok(qv) => qv,
                    Err(e) => {
                        let _ = tx.send(Err(Status::internal(e.to_string()))).await;
                        continue;
                    }
                };

                match sqlx::query_as::<_, lib_proto::management::Employee>(&query)
                    .bind(&values)
                    .fetch_one(&mut conn)
                    .await {
                    Ok(employee) => {
                        if tx.send(Ok(employee)).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(Status::internal(e.to_string()))).await;
                        continue;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn change_employee_avatar(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeAvatarRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

    let mut conn = lib_core::database::aquire_connection(&self.db).await?;
    let mut trx = lib_core::database::start_transaction(&mut conn).await?;

    let (query, values) = Query::update()
        .table((Alias::new("management"), Alias::new("employee")))
        .value(
            Alias::new("avatar_id"),
            payload.avatar_photo.id,
        )
        .and_where(Alias::new("id").eq(payload.employee_id))
        .map_err(lib_core::error::Error::Query)?
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&query, values)
        .execute(&mut *trx)
        .await
        .map_err(lib_core::error::Error::Database)?;

    lib_core::database::commit_transaction(trx).await?;

    return Ok(Response::new(()));
    }

    async fn change_employee_cover_photo(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeCoverPhotoRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("cover_photo_id"),
                payload.cover_photo.id,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_first_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeFirstNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("first_name"),
                payload.first_name,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_middle_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeMiddleNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("middle_name"),
                payload.middle_name,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_last_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeLastNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("last_name"),
                payload.last_name,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_tel_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeTelNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("tel_number"),
                payload.tel_number,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_mobile_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeMobileNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("mobile_number"),
                payload.mobile_number,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_email(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeEmailRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("email"),
                payload.email,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_role(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeRoleRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let role_str = match payload.role {
            0 => "SuperAdmin",
            1 => "Manager",
            2 => "Employee",
            _ => return Err(Status::invalid_argument("Invalid role value")),
        };

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("role"),
                role_str,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_status(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeStatusRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let status_str = match payload.status {
            0 => "Active",
            1 => "Inactive",
            _ => return Err(Status::invalid_argument("Invalid status value")),
        };

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("status"),
                status_str,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_employee_contract_type(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeContractTypeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let contract_type_str = match payload.contract_type {
            0 => "FullTime",
            1 => "PartTime",
            _ => return Err(Status::invalid_argument("Invalid contract type value")),
        };

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("contract_type"),
                contract_type_str,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_phil_nat_id(
        &self,
        request: tonic::Request<lib_proto::management::ChangePhilNatIDRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("phil_nat_id"),
                payload.phil_nat_id,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_birth_date(
        &self,
        request: tonic::Request<lib_proto::management::ChangeBirthDateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("birth_date"),
                payload.birth_date,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_spouse_first_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseFirstNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("spouse_first_name"),
                payload.spouse_first_name,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_spouse_middle_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseMiddleNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("spouse_middle_name"),
                payload.spouse_middle_name,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_spouse_last_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseLastNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("spouse_last_name"),
                payload.spouse_last_name,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn change_spouse_employer(
        &self,
        request: tonic::Request<lib_proto::management::ChangeSpouseEmployerRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;
        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        let (query, values) = Query::update()
            .table((Alias::new("management"), Alias::new("employee")))
            .value(
                Alias::new("spouse_employer"),
                payload.spouse_employer,
            )
            .and_where(Alias::new("id").eq(payload.employee_id))
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        return Ok(Response::new(()));
    }

    async fn remove_employee(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();
        let employee_id = req.id;

        if employee_id.is_empty() {
            return Err(tonic::Status::invalid_argument("Employee ID cannot be empty"));
        }

        match self.db.delete_employee(&employee_id).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to remove employee: {}", e))),
        }
    }

    async fn remove_special_interest(
        &self,
        request: tonic::Request<lib_proto::management::RemoveSpecialInterestRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();
        let special_interest = req.special_interest;

        let employee_id = match self.auth.get_current_employee_id() {
            Some(id) => id,
            None => return Err(tonic::Status::unauthenticated("Employee ID not found in authentication context")),
        };

        match self.db.remove_special_interest(&employee_id, &special_interest).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to remove special interest: {}", e))),
        }
    }

    async fn remove_learning_institition(
        &self,
        request: tonic::Request<lib_proto::management::RemoveLearningInstitutionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();
        let learning_institution = req.learning_institution;

        let employee_id = match self.auth.get_current_employee_id() {
            Some(id) => id,
            None => return Err(tonic::Status::unauthenticated("Employee ID not found in authentication context")),
        };

        match self.db.remove_learning_institution(&employee_id, &learning_institution).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to remove learning institution: {}", e))),
        }
    }

    async fn create_department(
        &self,
        request: tonic::Request<lib_proto::management::CreateDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();


        if req.name.is_empty() {
            return Err(tonic::Status::invalid_argument("Department name cannot be empty"));
        }

        let department = lib_proto::management::Department {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            description: req.description,
            employee_ids: Vec::new(),
        };

        match self.db.create_department(department).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to create department: {}", e))),
        }
    }

    async fn add_employee_to_department(
        &self,
        request: tonic::Request<lib_proto::management::AddEmployeeToDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        if req.employee_id.is_empty() || req.department_id.is_empty() {
            return Err(tonic::Status::invalid_argument("Employee ID and Department ID cannot be empty"));
        }

        if !self.db.employee_exists(&req.employee_id).await {
            return Err(tonic::Status::not_found(format!("Employee with ID {} not found", req.employee_id)));
        }

        let mut department = match self.db.get_department(&req.department_id).await {
            Ok(dept) => dept,
            Err(_) => return Err(tonic::Status::not_found(format!("Department with ID {} not found", req.department_id))),
        };

        if !department.employee_ids.contains(&req.employee_id) {
            department.employee_ids.push(req.employee_id);

            match self.db.update_department(department).await {
                Ok(_) => Ok(tonic::Response::new(())),
                Err(e) => Err(tonic::Status::internal(format!("Failed to add employee to department: {}", e))),
            }
        } else {
            Ok(tonic::Response::new(()))
        }
    }

    async fn get_department(
        &self,
        request: tonic::Request<lib_proto::management::GetDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::Department>, tonic::Status> {
        let req = request.into_inner();

        if req.id.is_empty() {
            return Err(tonic::Status::invalid_argument("Department ID cannot be empty"));
        }

        match self.db.get_department(&req.id).await {
            Ok(department) => Ok(tonic::Response::new(department)),
            Err(_) => Err(tonic::Status::not_found(format!("Department with ID {} not found", req.id))),
        }
    }

    async fn update_department_name(
        &self,
        request: tonic::Request<lib_proto::management::UpdateDepartmentNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let department_id = match self.context.get_department_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Department ID not found in context")),
        };

        let req = request.into_inner();

        if req.name.is_empty() {
            return Err(tonic::Status::invalid_argument("Department name cannot be empty"));
        }

        let mut department = match self.db.get_department(&department_id).await {
            Ok(dept) => dept,
            Err(_) => return Err(tonic::Status::not_found(format!("Department with ID {} not found", department_id))),
        };

        department.name = req.name;

        match self.db.update_department(department).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update department name: {}", e))),
        }
    }

    async fn update_department_description(
        &self,
        request: tonic::Request<lib_proto::management::UpdateDepartmentDescriptionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let department_id = match self.context.get_department_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Department ID not found in context")),
        };

        let req = request.into_inner();

        let mut department = match self.db.get_department(&department_id).await {
            Ok(dept) => dept,
            Err(_) => return Err(tonic::Status::not_found(format!("Department with ID {} not found", department_id))),
        };

        department.description = Some(req.description);

        match self.db.update_department(department).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update department description: {}", e))),
        }
    }

    async fn remove_department(
        &self,
        request: tonic::Request<lib_proto::management::RemoveDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        if req.id.is_empty() {
            return Err(tonic::Status::invalid_argument("Department ID cannot be empty"));
        }

        let department = match self.db.get_department(&req.id).await {
            Ok(dept) => dept,
            Err(_) => return Err(tonic::Status::not_found(format!("Department with ID {} not found", req.id))),
        };

        if !department.employee_ids.is_empty() {
            return Err(tonic::Status::failed_precondition("Cannot remove department with employees. Remove all employees first."));
        }

        match self.db.delete_department(&req.id).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to remove department: {}", e))),
        }
    }

    async fn remove_employee_to_department(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmployeeFromDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        if req.employee_id.is_empty() || req.department_id.is_empty() {
            return Err(tonic::Status::invalid_argument("Employee ID and Department ID cannot be empty"));
        }

        let mut department = match self.db.get_department(&req.department_id).await {
            Ok(dept) => dept,
            Err(_) => return Err(tonic::Status::not_found(format!("Department with ID {} not found", req.department_id))),
        };

        department.employee_ids.retain(|id| id != &req.employee_id);

        match self.db.update_department(department).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to remove employee from department: {}", e))),
        }
    }

    async fn create_job_information(
        &self,
        request: tonic::Request<lib_proto::management::CreateJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        if req.employee_id.is_empty() || req.department_id.is_empty() || req.title.is_empty() {
            return Err(tonic::Status::invalid_argument("Employee ID, Department ID, and Job Title cannot be empty"));
        }

        if !self.db.employee_exists(&req.employee_id).await {
            return Err(tonic::Status::not_found(format!("Employee with ID {} not found", req.employee_id)));
        }

        if !self.db.department_exists(&req.department_id).await {
            return Err(tonic::Status::not_found(format!("Department with ID {} not found", req.department_id)));
        }

        if !req.supervisor_id.is_empty() && !self.db.employee_exists(&req.supervisor_id).await {
            return Err(tonic::Status::not_found(format!("Supervisor with ID {} not found", req.supervisor_id)));
        }

        let job_info = lib_proto::management::JobInformation {
            id: uuid::Uuid::new_v4().to_string(),
            title: req.title,
            employee_id: req.employee_id,
            department_id: req.department_id,
            supervisor_id: req.supervisor_id,
            work_location: req.work_location,
            start_date: req.start_date,
            salary: req.salary,
            currency: req.currency,
        };

        match self.db.create_job_information(job_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to create job information: {}", e))),
        }
    }

    async fn get_job_information(
        &self,
        request: tonic::Request<lib_proto::management::GetJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::JobInformation>, tonic::Status> {
        let req = request.into_inner();

        if req.id.is_empty() {
            return Err(tonic::Status::invalid_argument("Job Information ID cannot be empty"));
        }

        match self.db.get_job_information(&req.id).await {
            Ok(job_info) => Ok(tonic::Response::new(job_info)),
            Err(_) => Err(tonic::Status::not_found(format!("Job Information with ID {} not found", req.id))),
        }
    }

    async fn change_job_title(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobTitleRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let job_id = match self.context.get_job_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Job ID not found in context")),
        };

        let req = request.into_inner();

        if req.title.is_empty() {
            return Err(tonic::Status::invalid_argument("Job title cannot be empty"));
        }

        let mut job_info = match self.db.get_job_information(&job_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Job information with ID {} not found", job_id))),
        };

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: job_info.employee_id.clone(),
            action_type: lib_proto::management::EmployeePanActionType::EmployeePanActionTypePromotion as i32,
            old_value: job_info.title.clone(),
            new_value: req.title.clone(),
            effective_date: chrono::Utc::now().to_rfc3339(),
            status: lib_proto::management::EmployeePanActionStatus::EmployeePanActionStatusPending as i32,
            requested_by: self.auth.get_current_user_id().unwrap_or_default(),
            approved_by: String::new(),
        };


        match self.db.create_pan(pan).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to create PAN record: {}", e))),
        }

        job_info.title = req.title;

        match self.db.update_job_information(job_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update job title: {}", e))),
        }
    }

    async fn change_job_department(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let job_id = match self.context.get_job_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Job ID not found in context")),
        };

        let req = request.into_inner();

        if req.department_id.is_empty() {
            return Err(tonic::Status::invalid_argument("Department ID cannot be empty"));
        }

        if !self.db.department_exists(&req.department_id).await {
            return Err(tonic::Status::not_found(format!("Department with ID {} not found", req.department_id)));
        }

        let mut job_info = match self.db.get_job_information(&job_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Job information with ID {} not found", job_id))),
        };

        let old_department = match self.db.get_department(&job_info.department_id).await {
            Ok(dept) => dept.name,
            Err(_) => "Unknown Department".to_string(),
        };

        let new_department = match self.db.get_department(&req.department_id).await {
            Ok(dept) => dept.name,
            Err(_) => "Unknown Department".to_string(),
        };

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: job_info.employee_id.clone(),
            action_type: lib_proto::management::EmployeePanActionType::EmployeePanActionTypeTransfer as i32,
            old_value: old_department,
            new_value: new_department,
            effective_date: chrono::Utc::now().to_rfc3339(),
            status: lib_proto::management::EmployeePanActionStatus::EmployeePanActionStatusPending as i32,
            requested_by: self.auth.get_current_user_id().unwrap_or_default(),
            approved_by: String::new(),
        };

        match self.db.create_pan(pan).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to create PAN record: {}", e))),
        }

        let mut old_dept = match self.db.get_department(&job_info.department_id).await {
            Ok(dept) => dept,
            Err(_) => return Err(tonic::Status::not_found(format!("Old department with ID {} not found", job_info.department_id))),
        };
        old_dept.employee_ids.retain(|id| id != &job_info.employee_id);
        match self.db.update_department(old_dept).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to update old department: {}", e))),
        }

        let mut new_dept = match self.db.get_department(&req.department_id).await {
            Ok(dept) => dept,
            Err(_) => return Err(tonic::Status::not_found(format!("New department with ID {} not found", req.department_id))),
        };
        if !new_dept.employee_ids.contains(&job_info.employee_id) {
            new_dept.employee_ids.push(job_info.employee_id.clone());
        }
        match self.db.update_department(new_dept).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to update new department: {}", e))),
        }

        job_info.department_id = req.department_id;

        match self.db.update_job_information(job_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update job department: {}", e))),
        }
    }

    async fn change_job_supervisor(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobSupervisorRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let job_id = match self.context.get_job_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Job ID not found in context")),
        };

        let req = request.into_inner();

        if !req.supervisor_id.is_empty() && !self.db.employee_exists(&req.supervisor_id).await {
            return Err(tonic::Status::not_found(format!("Supervisor with ID {} not found", req.supervisor_id)));
        }

        let mut job_info = match self.db.get_job_information(&job_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Job information with ID {} not found", job_id))),
        };

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: job_info.employee_id.clone(),
            action_type: lib_proto::management::EmployeePanActionType::EmployeePanActionTypeTransfer as i32,
            old_value: job_info.supervisor_id.clone(),
            new_value: req.supervisor_id.clone(),
            effective_date: chrono::Utc::now().to_rfc3339(),
            status: lib_proto::management::EmployeePanActionStatus::EmployeePanActionStatusPending as i32,
            requested_by: self.auth.get_current_user_id().unwrap_or_default(),
            approved_by: String::new(),
        };

        match self.db.create_pan(pan).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to create PAN record: {}", e))),
        }

        job_info.supervisor_id = req.supervisor_id;

        match self.db.update_job_information(job_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update job supervisor: {}", e))),
        }
    }

    async fn change_job_work_location(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobWorkLocationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let job_id = match self.context.get_job_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Job ID not found in context")),
        };

        let req = request.into_inner();

        if req.work_location.is_empty() {
            return Err(tonic::Status::invalid_argument("Work location cannot be empty"));
        }

        let mut job_info = match self.db.get_job_information(&job_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Job information with ID {} not found", job_id))),
        };

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: job_info.employee_id.clone(),
            action_type: lib_proto::management::EmployeePanActionType::EmployeePanActionTypeTransfer as i32,
            old_value: job_info.work_location.clone(),
            new_value: req.work_location.clone(),
            effective_date: chrono::Utc::now().to_rfc3339(),
            status: lib_proto::management::EmployeePanActionStatus::EmployeePanActionStatusPending as i32,
            requested_by: self.auth.get_current_user_id().unwrap_or_default(),
            approved_by: String::new(),
        };

        match self.db.create_pan(pan).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to create PAN record: {}", e))),
        }

        job_info.work_location = req.work_location;

        match self.db.update_job_information(job_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update work location: {}", e))),
        }
    }

    async fn change_job_start_date(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobStartDateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let job_id = match self.context.get_job_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Job ID not found in context")),
        };

        let req = request.into_inner();

        if req.start_date.is_empty() {
            return Err(tonic::Status::invalid_argument("Start date cannot be empty"));
        }

        match chrono::DateTime::parse_from_rfc3339(&req.start_date) {
            Ok(_) => (),
            Err(_) => return Err(tonic::Status::invalid_argument("Invalid start date format. Use RFC3339 format.")),
        }

        let mut job_info = match self.db.get_job_information(&job_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Job information with ID {} not found", job_id))),
        };

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: job_info.employee_id.clone(),
            action_type: lib_proto::management::EmployeePanActionType::EmployeePanActionTypeHire as i32,
            old_value: job_info.start_date.clone(),
            new_value: req.start_date.clone(),
            effective_date: chrono::Utc::now().to_rfc3339(),
            status: lib_proto::management::EmployeePanActionStatus::EmployeePanActionStatusPending as i32,
            requested_by: self.auth.get_current_user_id().unwrap_or_default(),
            approved_by: String::new(),
        };

        match self.db.create_pan(pan).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to create PAN record: {}", e))),
        }

        job_info.start_date = req.start_date;

        match self.db.update_job_information(job_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update start date: {}", e))),
        }
    }

    async fn change_job_salary(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobSalaryRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let job_id = match self.context.get_job_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Job ID not found in context")),
        };

        let req = request.into_inner();

        if req.salary.is_empty() {
            return Err(tonic::Status::invalid_argument("Salary cannot be empty"));
        }

        match req.salary.parse::<f64>() {
            Ok(_) => (),
            Err(_) => return Err(tonic::Status::invalid_argument("Salary must be a valid number")),
        }

        let mut job_info = match self.db.get_job_information(&job_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Job information with ID {} not found", job_id))),
        };

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: job_info.employee_id.clone(),
            action_type: lib_proto::management::EmployeePanActionType::EmployeePanActionTypeSalaryAdjustment as i32,
            old_value: job_info.salary.clone(),
            new_value: req.salary.clone(),
            effective_date: chrono::Utc::now().to_rfc3339(),
            status: lib_proto::management::EmployeePanActionStatus::EmployeePanActionStatusPending as i32,
            requested_by: self.auth.get_current_user_id().unwrap_or_default(),
            approved_by: String::new(),
        };

        match self.db.create_pan(pan).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to create PAN record: {}", e))),
        }

        job_info.salary = req.salary;

        match self.db.update_job_information(job_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update salary: {}", e))),
        }
    }

    async fn change_job_currency(
        &self,
        request: tonic::Request<lib_proto::management::ChangeJobCurrencyRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let job_id = match self.context.get_job_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Job ID not found in context")),
        };

        let req = request.into_inner();

        if req.currency.is_empty() {
            return Err(tonic::Status::invalid_argument("Currency cannot be empty"));
        }

        let mut job_info = match self.db.get_job_information(&job_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Job information with ID {} not found", job_id))),
        };

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: job_info.employee_id.clone(),
            action_type: lib_proto::management::EmployeePanActionType::EmployeePanActionTypeSalaryAdjustment as i32,
            old_value: job_info.currency.clone(),
            new_value: req.currency.clone(),
            effective_date: chrono::Utc::now().to_rfc3339(),
            status: lib_proto::management::EmployeePanActionStatus::EmployeePanActionStatusPending as i32,
            requested_by: self.auth.get_current_user_id().unwrap_or_default(),
            approved_by: String::new(),
        };

        match self.db.create_pan(pan).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to create PAN record: {}", e))),
        }

        job_info.currency = req.currency;

        match self.db.update_job_information(job_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update currency: {}", e))),
        }
    }

    async fn remove_job_information(
        &self,
        request: tonic::Request<lib_proto::management::RemoveJobInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        if req.id.is_empty() {
            return Err(tonic::Status::invalid_argument("Job information ID cannot be empty"));
        }

        if !self.auth.is_admin_or_manager() {
            return Err(tonic::Status::permission_denied("Only admins or managers can remove job information"));
        }

        let job_info = match self.db.get_job_information(&req.id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Job information with ID {} not found", req.id))),
        };

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: job_info.employee_id.clone(),
            action_type: lib_proto::management::EmployeePanActionType::EmployeePanActionTypeTermination as i32,
            old_value: format!("Position: {}", job_info.title),
            new_value: "Terminated".to_string(),
            effective_date: chrono::Utc::now().to_rfc3339(),
            status: lib_proto::management::EmployeePanActionStatus::EmployeePanActionStatusPending as i32,
            requested_by: self.auth.get_current_user_id().unwrap_or_default(),
            approved_by: String::new(),
        };

        match self.db.create_pan(pan).await {
            Ok(_) => (),
            Err(e) => return Err(tonic::Status::internal(format!("Failed to create PAN record: {}", e))),
        }

        match self.db.delete_job_information(&req.id).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to remove job information: {}", e))),
        }
    }

    async fn create_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmergencyInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        if req.employee_id.is_empty() || req.address.is_empty() || req.contact_name.is_empty() {
            return Err(tonic::Status::invalid_argument("Employee ID, address, and contact name cannot be empty"));
        }

        if !self.db.employee_exists(&req.employee_id).await {
            return Err(tonic::Status::not_found(format!("Employee with ID {} not found", req.employee_id)));
        }

        if self.db.emergency_information_exists(&req.employee_id).await {
            return Err(tonic::Status::already_exists(format!("Emergency information already exists for employee with ID {}", req.employee_id)));
        }

        let emergency_info = lib_proto::management::EmployeeEmergencyInformation {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: req.employee_id,
            address: req.address,
            tel_number: req.tel_number,
            mobile_number: req.mobile_number,
            health_conditions: req.health_conditions,
            contact_name: req.contact_name,
        };

        match self.db.create_emergency_information(emergency_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to create emergency information: {}", e))),
        }
    }

    async fn add_emergency_information_health_condition(
        &self,
        request: tonic::Request<lib_proto::management::AddEmergencyInformationHealthConditionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        if req.health_condition.is_empty() {
            return Err(tonic::Status::invalid_argument("Health condition cannot be empty"));
        }

        let employee_id = match self.context.get_employee_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Employee ID not found in context")),
        };

        let mut emergency_info = match self.db.get_emergency_information(&employee_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Emergency information for employee with ID {} not found", employee_id))),
        };

        if !emergency_info.health_conditions.contains(&req.health_condition) {
            emergency_info.health_conditions.push(req.health_condition);
        }

        match self.db.update_emergency_information(emergency_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to add health condition: {}", e))),
        }
    }

    async fn get_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::GetEmergencyInformationRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::EmployeeEmergencyInformation>, tonic::Status> {
        let req = request.into_inner();

        if req.employee_id.is_empty() {
            return Err(tonic::Status::invalid_argument("Employee ID cannot be empty"));
        }

        let current_user_id = self.auth.get_current_user_id().unwrap_or_default();
        let is_authorized = self.auth.is_admin_or_manager() || current_user_id == req.employee_id;

        if !is_authorized {
            return Err(tonic::Status::permission_denied("Not authorized to view emergency information"));
        }

        match self.db.get_emergency_information(&req.employee_id).await {
            Ok(info) => Ok(tonic::Response::new(info)),
            Err(_) => Err(tonic::Status::not_found(format!("Emergency information for employee with ID {} not found", req.employee_id))),
        }
    }

    async fn change_emergency_information_address(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmergencyInformationAddressRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        if req.address.is_empty() {
            return Err(tonic::Status::invalid_argument("Address cannot be empty"));
        }

        let employee_id = match self.context.get_employee_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Employee ID not found in context")),
        };

        let mut emergency_info = match self.db.get_emergency_information(&employee_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Emergency information for employee with ID {} not found", employee_id))),
        };

        emergency_info.address = req.address;

        match self.db.update_emergency_information(emergency_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update address: {}", e))),
        }
    }

    async fn change_emergency_information_tel_number(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmergencyInformationTelNumberRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        let employee_id = match self.context.get_employee_id_from_context() {
            Some(id) => id,
            None => return Err(tonic::Status::invalid_argument("Employee ID not found in context")),
        };

        let mut emergency_info = match self.db.get_emergency_information(&employee_id).await {
            Ok(info) => info,
            Err(_) => return Err(tonic::Status::not_found(format!("Emergency information for employee with ID {} not found", employee_id))),
        };

        emergency_info.tel_number = Some(req.tel_number);

        match self.db.update_emergency_information(emergency_info).await {
            Ok(_) => Ok(tonic::Response::new(())),
            Err(e) => Err(tonic::Status::internal(format!("Failed to update telephone number: {}", e))),
        }
    }


    async fn change_emergency_information_mobile_number(
        &self,
        request: tonic::Request<
            lib_proto::management::ChangeEmergencyInformationMobileNumberRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let emergency_info = match self.db.emergency_information_by_mobile_number(&request.mobile_number).await {
            Ok(Some(info)) => info,
            Ok(None) => return Err(tonic::Status::not_found("Emergency information not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if let Err(e) = self.db.update_emergency_info_mobile_number(&emergency_info.employee_id, &request.mobile_number).await {
            return Err(tonic::Status::internal(format!("Failed to update mobile number: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn change_emergency_information_contact_name(
        &self,
        request: tonic::Request<
            lib_proto::management::ChangeEmergencyInformationContactNameRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let emergency_info = match self.db.get_emergency_information_by_employee_id(&request.employee_id).await {
            Ok(Some(info)) => info,
            Ok(None) => return Err(tonic::Status::not_found("Emergency information not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if let Err(e) = self.db.update_emergency_info_contact_name(&emergency_info.id, &request.contact_name).await {
            return Err(tonic::Status::internal(format!("Failed to update contact name: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn remove_emergency_information(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmergencyInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        if let Err(e) = self.db.get_employee(&request.employee_id).await {
            return Err(tonic::Status::not_found(format!("Employee not found: {}", e)));
        }

        if let Err(e) = self.db.delete_emergency_information(&request.employee_id).await {
            return Err(tonic::Status::internal(format!("Failed to remove emergency information: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn remove_emergency_information_health_condition(
        &self,
        request: tonic::Request<
            lib_proto::management::RemoveEmergencyInformationHealthConditionRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let emergency_info = match self.db.get_emergency_information_by_condition(&request.health_condition).await {
            Ok(Some(info)) => info,
            Ok(None) => return Err(tonic::Status::not_found("Health condition not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if let Err(e) = self.db.remove_health_condition(&emergency_info.id, &request.health_condition).await {
            return Err(tonic::Status::internal(format!("Failed to remove health condition: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn create_pan_employee_request(
        &self,
        request: tonic::Request<lib_proto::management::CreatePANRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        match self.db.get_employee(&request.employee_id).await {
            Ok(_) => {},
            Err(e) => return Err(tonic::Status::not_found(format!("Employee not found: {}", e))),
        }

        let pan = lib_proto::management::PersonnelAction {
            id: uuid::Uuid::new_v4().to_string(),
            employee_id: request.employee_id,
            action_type: request.action_type,
            old_value: request.old_value,
            new_value: request.new_value,
            effective_date: request.effective_date,
            status: request.status,
            requested_by: self.get_authenticated_user_id()?,
            approved_by: String::new(),
        };

        if let Err(e) = self.db.create_personnel_action(pan).await {
            return Err(tonic::Status::internal(format!("Failed to create PAN: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn get_pan_information(
        &self,
        request: tonic::Request<lib_proto::management::GetPANInformationRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::PersonnelAction>, tonic::Status> {
        let request = request.into_inner();

        match self.db.get_personnel_action(&request.id).await {
            Ok(Some(pan)) => Ok(tonic::Response::new(pan)),
            Ok(None) => Err(tonic::Status::not_found("Personnel Action Notice not found")),
            Err(e) => Err(tonic::Status::internal(format!("Database error: {}", e))),
        }
    }

    async fn change_pan_action_type(
        &self,
        request: tonic::Request<lib_proto::management::ChangePANActionTypeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let pan = match self.db.get_personnel_action(&request.id).await {
            Ok(Some(pan)) => pan,
            Ok(None) => return Err(tonic::Status::not_found("Personnel Action Notice not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if pan.status != lib_proto::management::EmployeePanActionStatus::EMPLOYEE_PAN_ACTION_STATUS_PENDING as i32 {
            return Err(tonic::Status::failed_precondition("Cannot modify a PAN that has already been approved or rejected"));
        }

        if let Err(e) = self.db.update_pan_action_type(&request.id, request.action_type).await {
            return Err(tonic::Status::internal(format!("Failed to update PAN action type: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn change_pan_old_value(
        &self,
        request: tonic::Request<lib_proto::management::ChangePANOldValueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let pan = match self.db.get_personnel_action(&request.id).await {
            Ok(Some(pan)) => pan,
            Ok(None) => return Err(tonic::Status::not_found("Personnel Action Notice not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if pan.status != lib_proto::management::EmployeePanActionStatus::EMPLOYEE_PAN_ACTION_STATUS_PENDING as i32 {
            return Err(tonic::Status::failed_precondition("Cannot modify a PAN that has already been approved or rejected"));
        }

        if let Err(e) = self.db.update_pan_old_value(&request.id, &request.old_value).await {
            return Err(tonic::Status::internal(format!("Failed to update PAN old value: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn change_pan_new_value(
        &self,
        request: tonic::Request<lib_proto::management::ChangePANNewValueRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let pan = match self.db.get_personnel_action(&request.id).await {
            Ok(Some(pan)) => pan,
            Ok(None) => return Err(tonic::Status::not_found("Personnel Action Notice not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if pan.status != lib_proto::management::EmployeePanActionStatus::EMPLOYEE_PAN_ACTION_STATUS_PENDING as i32 {
            return Err(tonic::Status::failed_precondition("Cannot modify a PAN that has already been approved or rejected"));
        }

        if let Err(e) = self.db.update_pan_new_value(&request.id, &request.new_value).await {
            return Err(tonic::Status::internal(format!("Failed to update PAN new value: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn change_pan_effective_date(
        &self,
        request: tonic::Request<lib_proto::management::ChangePANEffectiveDateRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let pan = match self.db.get_personnel_action(&request.id).await {
            Ok(Some(pan)) => pan,
            Ok(None) => return Err(tonic::Status::not_found("Personnel Action Notice not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if pan.status != lib_proto::management::EmployeePanActionStatus::EMPLOYEE_PAN_ACTION_STATUS_PENDING as i32 {
            return Err(tonic::Status::failed_precondition("Cannot modify a PAN that has already been approved or rejected"));
        }

        if let Err(e) = self.db.update_pan_effective_date(&request.id, &request.effective_date).await {
            return Err(tonic::Status::internal(format!("Failed to update PAN effective date: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn approve_pan(
        &self,
        request: tonic::Request<lib_proto::management::ApprovePANRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        let user_id = self.get_authenticated_user_id()?;

        let user = match self.db.get_employee_by_auth_id(&user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(tonic::Status::permission_denied("User not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if user.role != lib_proto::management::EmployeeRole::ROLE_SUPER_ADMIN as i32 &&
           user.role != lib_proto::management::EmployeeRole::ROLE_MANAGER as i32 {
            return Err(tonic::Status::permission_denied("Only managers and admins can approve PANs"));
        }

        let pan = match self.db.get_personnel_action(&request.id).await {
            Ok(Some(pan)) => pan,
            Ok(None) => return Err(tonic::Status::not_found("Personnel Action Notice not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if pan.status != lib_proto::management::EmployeePanActionStatus::EMPLOYEE_PAN_ACTION_STATUS_PENDING as i32 {
            return Err(tonic::Status::failed_precondition("PAN has already been processed"));
        }

        if let Err(e) = self.db.update_pan_status(
            &request.id,
            lib_proto::management::EmployeePanActionStatus::EMPLOYEE_PAN_ACTION_STATUS_APPROVED,
            &user_id
        ).await {
            return Err(tonic::Status::internal(format!("Failed to approve PAN: {}", e)));
        }

        if pan.action_type == lib_proto::management::EmployeePanActionType::EMPLOYEE_PAN_ACTION_TYPE_TERMINATION as i32 {
            if let Err(e) = self.db.update_employee_status(&pan.employee_id, lib_proto::management::EmployeeStatus::EMPLOYEE_STATUS_INACTIVE).await {
                return Err(tonic::Status::internal(format!("Failed to update employee status: {}", e)));
            }
        }

        Ok(tonic::Response::new(()))
    }

    async fn reject_pan(
        &self,
        request: tonic::Request<lib_proto::management::RejectPANRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        let user_id = self.get_authenticated_user_id()?;

        let user = match self.db.get_employee_by_auth_id(&user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(tonic::Status::permission_denied("User not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if user.role != lib_proto::management::EmployeeRole::ROLE_SUPER_ADMIN as i32 &&
           user.role != lib_proto::management::EmployeeRole::ROLE_MANAGER as i32 {
            return Err(tonic::Status::permission_denied("Only managers and admins can reject PANs"));
        }

        let pan = match self.db.get_personnel_action(&request.id).await {
            Ok(Some(pan)) => pan,
            Ok(None) => return Err(tonic::Status::not_found("Personnel Action Notice not found")),
            Err(e) => return Err(tonic::Status::internal(format!("Database error: {}", e))),
        };

        if pan.status != lib_proto::management::EmployeePanActionStatus::EMPLOYEE_PAN_ACTION_STATUS_PENDING as i32 {
            return Err(tonic::Status::failed_precondition("PAN has already been processed"));
        }

        if let Err(e) = self.db.update_pan_status(
            &request.id,
            lib_proto::management::EmployeePanActionStatus::EMPLOYEE_PAN_ACTION_STATUS_REJECTED,
            &user_id
        ).await {
            return Err(tonic::Status::internal(format!("Failed to reject PAN: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    async fn remove_pan_information(
        &self,
        request: tonic::Request<lib_proto::management::RemovePANInformationRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        if let Err(e) = self.db.get_personnel_action(&request.id).await {
            return Err(tonic::Status::not_found(format!("Personnel Action Notice not found: {}", e)));
        }

        if let Err(e) = self.db.delete_personnel_action(&request.id).await {
            return Err(tonic::Status::internal(format!("Failed to remove PAN: {}", e)));
        }

        Ok(tonic::Response::new(()))
    }

    fn get_authenticated_user_id(&self) -> Result<String, tonic::Status> {
        match self.auth_context.get_user_id() {
            Some(user_id) => Ok(user_id),
            None => Err(tonic::Status::unauthenticated("User not authenticated")),
        }
    }
}
*/
