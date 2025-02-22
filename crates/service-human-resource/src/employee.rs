use lib_proto::management::{
    human_resource_service_server::HumanResourceService as GrpcHumanResourceService, Employee,
};
use sqlx::{Pool, Postgres};
use tokio_stream::wrappers::ReceiverStream;
use tonic::Status;

pub struct HumanResourceService {
    db: Pool<Postgres>,
}

#[tonic::async_trait]
impl GrpcHumanResourceService for HumanResourceService {
    async fn create_employee(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let payload = request.into_inner();

        let mut conn = lib_core::database::aquire_connection(&self.db).await?;

        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        lib_core::database::commit_transaction(trx).await?;

        todo!()
    }

    async fn create_manager(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn create_admin(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn add_special_interest(
        &self,
        request: tonic::Request<lib_proto::management::AddSpecialInterestRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn add_learning_institution(
        &self,
        request: tonic::Request<lib_proto::management::AddLearningInstitutionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn get_employee(
        &self,
        request: tonic::Request<lib_proto::management::GetEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::Employee>, tonic::Status> {
        todo!()
    }

    type GetEmployeesByDepartmentStream = ReceiverStream<Result<Employee, Status>>;

    async fn get_employees_by_department(
        &self,
        request: tonic::Request<lib_proto::management::GetEmployeesByDepartmentRequest>,
    ) -> std::result::Result<tonic::Response<Self::GetEmployeesByDepartmentStream>, tonic::Status>
    {
        todo!()
    }

    async fn change_employee_avatar(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeAvatarRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_employee_cover_photo(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeCoverPhotoRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }

    async fn change_employee_first_name(
        &self,
        request: tonic::Request<lib_proto::management::ChangeEmployeeFirstNameRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
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
        todo!()
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
