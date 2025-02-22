use lib_proto::management::{
    employee_service_server::EmployeeService as GrpcEmployeeService, Employee,
};
use sqlx::{Pool, Postgres};
use tokio_stream::wrappers::ReceiverStream;
use tonic::Status;

pub struct EmployeeService {
    db: Pool<Postgres>,
}

#[tonic::async_trait]
impl GrpcEmployeeService for EmployeeService {
    type GetEmployeeStream = ReceiverStream<Result<Employee, Status>>;
    type BatchGetEmployeesStream = ReceiverStream<Result<Employee, Status>>;

    async fn create_employee(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<
        tonic::Response<lib_proto::management::CreateEmployeeResponse>,
        tonic::Status,
    > {
        unimplemented!()
    }

    async fn get_employee(
        &self,
        request: tonic::Request<lib_proto::management::CreateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<Self::GetEmployeeStream>, tonic::Status> {
        unimplemented!()
    }

    async fn batch_get_employees(
        &self,
        request: tonic::Request<lib_proto::management::BatchGetEmployeesRequest>,
    ) -> std::result::Result<tonic::Response<Self::BatchGetEmployeesStream>, tonic::Status> {
        unimplemented!()
    }

    async fn update_employee(
        &self,
        request: tonic::Request<lib_proto::management::UpdateEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<lib_proto::management::Employee>, tonic::Status> {
        unimplemented!()
    }

    async fn remove_employee(
        &self,
        request: tonic::Request<lib_proto::management::RemoveEmployeeRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        unimplemented!()
    }
}
