// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMetadata {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub size: u32,
    #[prost(bool, tag = "5")]
    pub is_public: bool,
    #[prost(string, tag = "6")]
    pub owner_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileChunk {
    #[prost(bytes = "vec", tag = "1")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFileMetadataRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub size: u32,
    #[prost(bool, tag = "4")]
    pub is_public: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFileRequest {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<CreateFileMetadataRequest>,
    #[prost(message, optional, tag = "2")]
    pub chunk: ::core::option::Option<FileChunk>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadFileRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMetadataRequest {
    #[prost(oneof = "file_metadata_request::Request", tags = "1, 2")]
    pub request: ::core::option::Option<file_metadata_request::Request>,
}
/// Nested message and enum types in `FileMetadataRequest`.
pub mod file_metadata_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(string, tag = "1")]
        Id(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Name(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareFileRequest {
    #[prost(string, tag = "1")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(oneof = "share_file_request::ShareOption", tags = "2")]
    pub share_option: ::core::option::Option<share_file_request::ShareOption>,
}
/// Nested message and enum types in `ShareFileRequest`.
pub mod share_file_request {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum ShareOption {
        #[prost(bool, tag = "2")]
        IsPublic(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFileRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod storage_service_client {
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
    pub struct StorageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageServiceClient<tonic::transport::Channel> {
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
    impl<T> StorageServiceClient<T>
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
        ) -> StorageServiceClient<InterceptedService<T, F>>
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
            StorageServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_file(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::CreateFileRequest>,
        ) -> std::result::Result<tonic::Response<super::FileMetadata>, tonic::Status> {
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
                "/storage.StorageService/CreateFile",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("storage.StorageService", "CreateFile"));
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn list_owned_files(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::FileMetadata>>,
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
                "/storage.StorageService/ListOwnedFiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("storage.StorageService", "ListOwnedFiles"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn list_shared_files(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::FileMetadata>>,
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
                "/storage.StorageService/ListSharedFiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("storage.StorageService", "ListSharedFiles"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn download_file(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadFileRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::FileChunk>>,
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
                "/storage.StorageService/DownloadFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("storage.StorageService", "DownloadFile"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_file_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::FileMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::FileMetadata>, tonic::Status> {
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
                "/storage.StorageService/GetFileMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("storage.StorageService", "GetFileMetadata"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn share_file(
            &mut self,
            request: impl tonic::IntoRequest<super::ShareFileRequest>,
        ) -> std::result::Result<tonic::Response<super::FileMetadata>, tonic::Status> {
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
                "/storage.StorageService/ShareFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("storage.StorageService", "ShareFile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_file(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFileRequest>,
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
                "/storage.StorageService/DeleteFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("storage.StorageService", "DeleteFile"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod storage_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StorageServiceServer.
    #[async_trait]
    pub trait StorageService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_file(
            &self,
            request: tonic::Request<tonic::Streaming<super::CreateFileRequest>>,
        ) -> std::result::Result<tonic::Response<super::FileMetadata>, tonic::Status>;
        /// Server streaming response type for the ListOwnedFiles method.
        type ListOwnedFilesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::FileMetadata, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_owned_files(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<Self::ListOwnedFilesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListSharedFiles method.
        type ListSharedFilesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::FileMetadata, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_shared_files(
            &self,
            request: tonic::Request<()>,
        ) -> std::result::Result<
            tonic::Response<Self::ListSharedFilesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DownloadFile method.
        type DownloadFileStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::FileChunk, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn download_file(
            &self,
            request: tonic::Request<super::DownloadFileRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::DownloadFileStream>,
            tonic::Status,
        >;
        async fn get_file_metadata(
            &self,
            request: tonic::Request<super::FileMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::FileMetadata>, tonic::Status>;
        async fn share_file(
            &self,
            request: tonic::Request<super::ShareFileRequest>,
        ) -> std::result::Result<tonic::Response<super::FileMetadata>, tonic::Status>;
        async fn delete_file(
            &self,
            request: tonic::Request<super::DeleteFileRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StorageServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> StorageServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StorageServiceServer<T>
    where
        T: StorageService,
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
                "/storage.StorageService/CreateFile" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFileSvc<T: StorageService>(pub Arc<T>);
                    impl<
                        T: StorageService,
                    > tonic::server::ClientStreamingService<super::CreateFileRequest>
                    for CreateFileSvc<T> {
                        type Response = super::FileMetadata;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::CreateFileRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageService>::create_file(&inner, request).await
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
                        let method = CreateFileSvc(inner);
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
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/storage.StorageService/ListOwnedFiles" => {
                    #[allow(non_camel_case_types)]
                    struct ListOwnedFilesSvc<T: StorageService>(pub Arc<T>);
                    impl<T: StorageService> tonic::server::ServerStreamingService<()>
                    for ListOwnedFilesSvc<T> {
                        type Response = super::FileMetadata;
                        type ResponseStream = T::ListOwnedFilesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageService>::list_owned_files(&inner, request)
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
                        let method = ListOwnedFilesSvc(inner);
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
                "/storage.StorageService/ListSharedFiles" => {
                    #[allow(non_camel_case_types)]
                    struct ListSharedFilesSvc<T: StorageService>(pub Arc<T>);
                    impl<T: StorageService> tonic::server::ServerStreamingService<()>
                    for ListSharedFilesSvc<T> {
                        type Response = super::FileMetadata;
                        type ResponseStream = T::ListSharedFilesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageService>::list_shared_files(&inner, request)
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
                        let method = ListSharedFilesSvc(inner);
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
                "/storage.StorageService/DownloadFile" => {
                    #[allow(non_camel_case_types)]
                    struct DownloadFileSvc<T: StorageService>(pub Arc<T>);
                    impl<
                        T: StorageService,
                    > tonic::server::ServerStreamingService<super::DownloadFileRequest>
                    for DownloadFileSvc<T> {
                        type Response = super::FileChunk;
                        type ResponseStream = T::DownloadFileStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageService>::download_file(&inner, request).await
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
                        let method = DownloadFileSvc(inner);
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
                "/storage.StorageService/GetFileMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetFileMetadataSvc<T: StorageService>(pub Arc<T>);
                    impl<
                        T: StorageService,
                    > tonic::server::UnaryService<super::FileMetadataRequest>
                    for GetFileMetadataSvc<T> {
                        type Response = super::FileMetadata;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FileMetadataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageService>::get_file_metadata(&inner, request)
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
                        let method = GetFileMetadataSvc(inner);
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
                "/storage.StorageService/ShareFile" => {
                    #[allow(non_camel_case_types)]
                    struct ShareFileSvc<T: StorageService>(pub Arc<T>);
                    impl<
                        T: StorageService,
                    > tonic::server::UnaryService<super::ShareFileRequest>
                    for ShareFileSvc<T> {
                        type Response = super::FileMetadata;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShareFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageService>::share_file(&inner, request).await
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
                        let method = ShareFileSvc(inner);
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
                "/storage.StorageService/DeleteFile" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteFileSvc<T: StorageService>(pub Arc<T>);
                    impl<
                        T: StorageService,
                    > tonic::server::UnaryService<super::DeleteFileRequest>
                    for DeleteFileSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteFileRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as StorageService>::delete_file(&inner, request).await
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
                        let method = DeleteFileSvc(inner);
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
    impl<T> Clone for StorageServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "storage.StorageService";
    impl<T> tonic::server::NamedService for StorageServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
