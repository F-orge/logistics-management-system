use std::io;

use axum::{
    body::{Body, Bytes},
    extract::{Multipart, Path},
    http::StatusCode,
    routing::{get, post},
    BoxError, Router,
};
use futures::{Stream, TryStreamExt};
use sea_orm::DatabaseConnection;
use sqlx::types::Uuid;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::{ReaderStream, StreamReader};

use crate::models::_proto::employee_management::file_service_server::{
    FileService as GrpcFileService, FileServiceServer,
};

#[derive(Default)]
pub struct FileService {
    db: DatabaseConnection,
}

impl FileService {
    pub fn new(db: &DatabaseConnection) -> (Router, FileServiceServer<Self>) {
        let axum_router = Router::new()
            .route("/", post(FileService::upload_file))
            .route("/", get(FileService::retrieve_file));

        let grpc_router = FileServiceServer::new(Self { db: db.clone() });

        (axum_router, grpc_router)
    }

    async fn upload_file(mut multipart: Multipart) -> Result<(), (StatusCode, String)> {
        // retrieve the file bytes and store it in a `storage` folder
        while let Ok(Some(field)) = multipart.next_field().await {
            // TODO: just replace this with Uuid::new_v4 and convert it to string since we do not need the file name
            // and we will compress it later
            let file_name = if let Some(file_name) = field.file_name() {
                file_name.to_owned()
            } else {
                continue;
            };
            match FileService::stream_to_file(&file_name, field).await {
                Ok(_) => {}
                Err(err) => return Err(err),
            };
        }
        Ok(())
    }

    async fn stream_to_file<S, E>(name: &str, stream: S) -> Result<(), (StatusCode, String)>
    where
        S: Stream<Item = Result<Bytes, E>>,
        E: Into<BoxError>,
    {
        // Check if file id exists in the folder
        // - return error if we have duplicate
        // TODO: compress the file
        // save the file to disk

        async {
            // Convert the stream into an `AsyncRead`.
            let body_with_io_error =
                stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
            let body_reader = StreamReader::new(body_with_io_error);
            futures::pin_mut!(body_reader);
            // Create the file. `File` implements `AsyncWrite`.
            // TODO: change the `storage` path &str to environment variable
            let path = std::path::Path::new("storage").join(name);
            let mut file = BufWriter::new(File::create(path).await?);
            // Copy the body into the file.
            tokio::io::copy(&mut body_reader, &mut file).await?;
            Ok::<_, io::Error>(())
        }
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }

    async fn retrieve_file(Path(id): Path<Uuid>) -> Result<Body, (StatusCode, String)> {
        // TODO: get the jwt token and check if we can give the file to the client.

        let file_path = std::path::Path::new("storage").join(id.to_string());

        let file = match tokio::fs::File::open(file_path).await {
            Ok(file) => file,
            Err(_) => return Err((StatusCode::NOT_FOUND, "File object not found".into())),
        };

        let stream = ReaderStream::new(file);

        let body = Body::from_stream(stream);

        // TODO: implement proper headers so that the client knows what file it will get

        Ok(body)
    }
}

#[tonic::async_trait]
impl GrpcFileService for FileService {
    async fn get_file(
        &self,
        request: tonic::Request<crate::models::_proto::employee_management::GetFileRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::models::_proto::employee_management::File>,
        tonic::Status,
    > {
        unimplemented!()
    }

    async fn delete_file(
        &self,
        request: tonic::Request<crate::models::_proto::employee_management::DeleteFileRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::models::_proto::employee_management::Empty>,
        tonic::Status,
    > {
        unimplemented!()
    }

    async fn update_file(
        &self,
        request: tonic::Request<crate::models::_proto::employee_management::UpdateFileRequest>,
    ) -> std::result::Result<
        tonic::Response<crate::models::_proto::employee_management::File>,
        tonic::Status,
    > {
        unimplemented!()
    }
}
