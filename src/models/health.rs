use health_check_server::HealthCheckServer;
use tonic_web::CorsGrpcWeb;

tonic::include_proto!("health_check");

#[derive(Debug, Default)]
pub struct HealthCheckService;

#[tonic::async_trait]
impl health_check_server::HealthCheck for HealthCheckService {
    async fn check(
        &self,
        _request: tonic::Request<HealthCheckRequest>,
    ) -> Result<tonic::Response<HealthCheckResponse>, tonic::Status> {
        let response = HealthCheckResponse {
            status: 200,
            message: "Service is up and running".into(),
        };
        Ok(tonic::Response::new(response))
    }
}

pub fn health_check_service() -> CorsGrpcWeb<HealthCheckServer<HealthCheckService>> {
    tonic_web::enable(HealthCheckServer::new(HealthCheckService::default()))
}
