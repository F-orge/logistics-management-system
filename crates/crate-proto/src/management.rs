// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Employee {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration = "Role", tag = "3")]
    pub role: i32,
    #[prost(string, tag = "4")]
    pub full_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub position: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub avatar_file_id: ::core::option::Option<super::storage::FileMetadata>,
    #[prost(message, optional, tag = "8")]
    pub cover_photo_file_id: ::core::option::Option<super::storage::FileMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Team {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub leader_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub member_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Board {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub team_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoardSection {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub color: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub hidden: bool,
    #[prost(int32, tag = "6")]
    pub task_limit: i32,
    #[prost(string, tag = "7")]
    pub task_board_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub board_section_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub fields: ::prost::alloc::vec::Vec<TaskField>,
    #[prost(message, repeated, tag = "6")]
    pub labels: ::prost::alloc::vec::Vec<TaskLabel>,
    #[prost(message, repeated, tag = "7")]
    pub comments: ::prost::alloc::vec::Vec<TaskComment>,
    #[prost(string, tag = "8")]
    pub issuer_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "9")]
    pub assignee_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskField {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "TaskFieldType", tag = "3")]
    pub r#type: i32,
    /// NOTE TO CLIENTS: please convert this into proper data type before using it.
    /// you can check the type first before doing the conversion
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskLabel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub color: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskComment {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// NOTE: this is a markdown compatible comment
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sender_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub attachments: ::prost::alloc::vec::Vec<super::storage::FileMetadata>,
    /// NOTE: this can be used to identify the order of the comments
    #[prost(string, tag = "6")]
    pub timestamp: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEmployeeRequest {
    #[prost(string, tag = "1")]
    pub full_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "Role", tag = "3")]
    pub role: i32,
    #[prost(string, tag = "4")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "6")]
    pub avatar_file_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub avatar_cover_photo_file_id: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEmployeeResponse {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// NOTE: this is a auto-generate password for new employees
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEmployeeRequest {
    #[prost(oneof = "get_employee_request::Request", tags = "1, 2, 3")]
    pub request: ::core::option::Option<get_employee_request::Request>,
}
/// Nested message and enum types in `GetEmployeeRequest`.
pub mod get_employee_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(string, tag = "1")]
        EmployeeId(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        UserId(::prost::alloc::string::String),
        #[prost(enumeration = "super::Role", tag = "3")]
        Role(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetEmployeesRequest {
    #[prost(string, repeated, tag = "1")]
    pub employee_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEmployeeRequest {
    #[prost(oneof = "update_employee_request::Request", tags = "3, 4, 5, 6, 9, 10")]
    pub request: ::core::option::Option<update_employee_request::Request>,
}
/// Nested message and enum types in `UpdateEmployeeRequest`.
pub mod update_employee_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(enumeration = "super::Role", tag = "3")]
        Role(i32),
        #[prost(string, tag = "4")]
        FullName(::prost::alloc::string::String),
        #[prost(string, tag = "5")]
        Address(::prost::alloc::string::String),
        #[prost(string, tag = "6")]
        Position(::prost::alloc::string::String),
        #[prost(message, tag = "9")]
        AvatarFile(super::super::storage::FileMetadata),
        #[prost(message, tag = "10")]
        ConverPhotoFile(super::super::storage::FileMetadata),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEmployeeRequest {
    #[prost(string, tag = "1")]
    pub employee_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Role {
    SuperAdmin = 0,
    Manager = 1,
    Employee = 2,
}
impl Role {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::SuperAdmin => "ROLE_SUPER_ADMIN",
            Self::Manager => "ROLE_MANAGER",
            Self::Employee => "ROLE_EMPLOYEE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROLE_SUPER_ADMIN" => Some(Self::SuperAdmin),
            "ROLE_MANAGER" => Some(Self::Manager),
            "ROLE_EMPLOYEE" => Some(Self::Employee),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractType {
    FullTime = 0,
    PartTime = 1,
}
impl ContractType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::FullTime => "CONTRACT_TYPE_FULL_TIME",
            Self::PartTime => "CONTRACT_TYPE_PART_TIME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTRACT_TYPE_FULL_TIME" => Some(Self::FullTime),
            "CONTRACT_TYPE_PART_TIME" => Some(Self::PartTime),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskFieldType {
    Text = 0,
    Number = 1,
    Date = 2,
}
impl TaskFieldType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Text => "TASK_FIELD_TYPE_TEXT",
            Self::Number => "TASK_FIELD_TYPE_NUMBER",
            Self::Date => "TASK_FIELD_TYPE_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TASK_FIELD_TYPE_TEXT" => Some(Self::Text),
            "TASK_FIELD_TYPE_NUMBER" => Some(Self::Number),
            "TASK_FIELD_TYPE_DATE" => Some(Self::Date),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod employee_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct EmployeeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EmployeeServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EmployeeServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> EmployeeServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            EmployeeServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_employee(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEmployeeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateEmployeeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/management.EmployeeService/CreateEmployee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.EmployeeService", "CreateEmployee"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_employee(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEmployeeRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Employee>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/management.EmployeeService/GetEmployee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.EmployeeService", "GetEmployee"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn batch_get_employees(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetEmployeesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Employee>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/management.EmployeeService/BatchGetEmployees",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("management.EmployeeService", "BatchGetEmployees"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn update_employee(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEmployeeRequest>,
        ) -> std::result::Result<tonic::Response<super::Employee>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/management.EmployeeService/UpdateEmployee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.EmployeeService", "UpdateEmployee"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_employee(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveEmployeeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/management.EmployeeService/RemoveEmployee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("management.EmployeeService", "RemoveEmployee"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod employee_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EmployeeServiceServer.
    #[async_trait]
    pub trait EmployeeService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_employee(
            &self,
            request: tonic::Request<super::CreateEmployeeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateEmployeeResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetEmployee method.
        type GetEmployeeStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Employee, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn get_employee(
            &self,
            request: tonic::Request<super::CreateEmployeeRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::GetEmployeeStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the BatchGetEmployees method.
        type BatchGetEmployeesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Employee, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn batch_get_employees(
            &self,
            request: tonic::Request<super::BatchGetEmployeesRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::BatchGetEmployeesStream>,
            tonic::Status,
        >;
        async fn update_employee(
            &self,
            request: tonic::Request<super::UpdateEmployeeRequest>,
        ) -> std::result::Result<tonic::Response<super::Employee>, tonic::Status>;
        async fn remove_employee(
            &self,
            request: tonic::Request<super::RemoveEmployeeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct EmployeeServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> EmployeeServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EmployeeServiceServer<T>
    where
        T: EmployeeService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/management.EmployeeService/CreateEmployee" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEmployeeSvc<T: EmployeeService>(pub Arc<T>);
                    impl<
                        T: EmployeeService,
                    > tonic::server::UnaryService<super::CreateEmployeeRequest>
                    for CreateEmployeeSvc<T> {
                        type Response = super::CreateEmployeeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEmployeeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EmployeeService>::create_employee(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateEmployeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/management.EmployeeService/GetEmployee" => {
                    #[allow(non_camel_case_types)]
                    struct GetEmployeeSvc<T: EmployeeService>(pub Arc<T>);
                    impl<
                        T: EmployeeService,
                    > tonic::server::ServerStreamingService<super::CreateEmployeeRequest>
                    for GetEmployeeSvc<T> {
                        type Response = super::Employee;
                        type ResponseStream = T::GetEmployeeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEmployeeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EmployeeService>::get_employee(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetEmployeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/management.EmployeeService/BatchGetEmployees" => {
                    #[allow(non_camel_case_types)]
                    struct BatchGetEmployeesSvc<T: EmployeeService>(pub Arc<T>);
                    impl<
                        T: EmployeeService,
                    > tonic::server::ServerStreamingService<
                        super::BatchGetEmployeesRequest,
                    > for BatchGetEmployeesSvc<T> {
                        type Response = super::Employee;
                        type ResponseStream = T::BatchGetEmployeesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchGetEmployeesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EmployeeService>::batch_get_employees(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BatchGetEmployeesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/management.EmployeeService/UpdateEmployee" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateEmployeeSvc<T: EmployeeService>(pub Arc<T>);
                    impl<
                        T: EmployeeService,
                    > tonic::server::UnaryService<super::UpdateEmployeeRequest>
                    for UpdateEmployeeSvc<T> {
                        type Response = super::Employee;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateEmployeeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EmployeeService>::update_employee(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateEmployeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/management.EmployeeService/RemoveEmployee" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveEmployeeSvc<T: EmployeeService>(pub Arc<T>);
                    impl<
                        T: EmployeeService,
                    > tonic::server::UnaryService<super::RemoveEmployeeRequest>
                    for RemoveEmployeeSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveEmployeeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EmployeeService>::remove_employee(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveEmployeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for EmployeeServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "management.EmployeeService";
    impl<T> tonic::server::NamedService for EmployeeServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
