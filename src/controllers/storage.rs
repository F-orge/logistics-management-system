use std::{
    path::{Path, PathBuf},
    pin::Pin,
};

use futures::{Stream, StreamExt};
use sqlx::{types::Uuid, Pool, Postgres};
use tokio::fs;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

use crate::models::_proto::{
    self,
    storage::{
        storage_service_server::{StorageService as GRPCStorageService, StorageServiceServer},
        FileChunk, FileMetadata,
    },
};

pub struct StorageService {
    db: Pool<Postgres>,
    directory: PathBuf,
}

impl StorageService {
    pub fn new(db: &Pool<Postgres>, directory: &Path) -> StorageServiceServer<Self> {
        StorageServiceServer::new(Self {
            db: db.clone(),
            directory: directory.to_path_buf(),
        })
    }
}

#[tonic::async_trait]
impl GRPCStorageService for StorageService {
    type DownloadFileStream = ReceiverStream<Result<FileChunk, Status>>;

    async fn create_file(
        &self,
        request: tonic::Request<tonic::Streaming<_proto::storage::CreateFileRequest>>,
    ) -> std::result::Result<tonic::Response<_proto::storage::FileMetadata>, tonic::Status> {
        // refer to this documentation. https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        // under client side streaming section.
        let mut stream = request.into_inner();

        let mut chunks = Vec::new();

        let mut metadata = FileMetadata::default();
        
        while let Some(chunk) = stream.next().await {
            // get the first chunk and get its metadata
            let file_chunk = match chunk {
                Ok(file_request) => {
                    metadata = match file_request.metadata {
                        Some(metadata) => metadata,
                        None => return Err(Status::invalid_argument("Cannot get file metadata")),
                    };
                    match file_request.chunk {
                        Some(chunk) => chunk.chunk,
                        None => return Err(Status::data_loss("Cannot get chunk")),
                    }
                }
                Err(err) => return Err(err),
            };
            chunks.push(file_chunk); 
        
        }

        let file_id = match metadata.id {
            Some(file_id) => file_id,
            None => Uuid::new_v4().to_string(),
        };

        let file_path = self
            .directory
            .join(format!("{}-{}", file_id, metadata.name));

        let file_contents = chunks.into_iter().flatten().collect::<Vec<u8>>();
        // check if file chunks have the same size as metadata.size
        if file_contents.len() != metadata.size as usize {
            return Err(Status::data_loss("Invalid file size"));
        }

        match fs::write(file_path, file_contents).await {
            Ok(_) => {}
            Err(_) => return Err(Status::internal("Cannot write file to the server")),
        };

        // save it first to the database
        let db_response = match sqlx::query!(
            r#"insert into "storage"."file" (name, type, size) values ($1, $2, $3) returning *"#,
            metadata.name,
            metadata.r#type,
            metadata.size as i32
        )
        .fetch_one(&self.db)
        .await
        {
            Ok(record) => record,
            Err(_) => return Err(Status::internal("Cannot insert file to database")),
        };

        Ok(Response::new(FileMetadata {
            id: Some(db_response.id.to_string()),
            name: db_response.name,
            r#type: db_response.r#type,
            size: db_response.size as u32,
        }))
    }

    async fn download_file(
        &self,
        request: tonic::Request<_proto::storage::DownloadFileRequest>,
    ) -> std::result::Result<tonic::Response<Self::DownloadFileStream>, tonic::Status> {
        unimplemented!()
    }

    async fn get_file_metadata(
        &self,
        request: tonic::Request<_proto::storage::FileMetadataRequest>,
    ) -> std::result::Result<tonic::Response<_proto::storage::FileMetadata>, tonic::Status> {
        unimplemented!()
    }

    async fn delete_file(
        &self,
        request: tonic::Request<_proto::storage::DeleteFileRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        unimplemented!()
    }

    async fn file_exists(
        &self,
        request: tonic::Request<_proto::storage::FileMetadataRequest>,
    ) -> std::result::Result<tonic::Response<_proto::storage::FileExistsResponse>, tonic::Status,> {
        unimplemented!()
    }

}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use tempdir::TempDir;
    use tonic::{transport::Server, Request};

    use crate::{
        models::_proto::storage::{
            storage_service_client::StorageServiceClient, CreateFileRequest,
        },
        utils::test::start_server,
    };

    use super::*;

    #[sqlx::test]
    async fn test_storage_create_file(db: Pool<Postgres>) {
        let tmp_dir = TempDir::new("temp_storage").unwrap();

        let (_, channel) =
            start_server(Server::builder().add_service(StorageService::new(&db, tmp_dir.path())))
                .await;

        let mut client = StorageServiceClient::new(channel);

        // send one chunk to the backend
        let file_content = b"HELLO MY NAME IS JOHN DOE. i am a file!!! :3";

        let file_metadata = CreateFileRequest {
            metadata: Some(FileMetadata {
                id: None,
                name: "my_file".into(),
                r#type: "text/plain".into(),
                size: file_content.len() as u32,
            }),
            chunk: Some(FileChunk {
                chunk: file_content.to_vec(),
            }),
            total_chunks: 1,      
            chunk_number: 1,
        };

        let request_stream = tokio_stream::iter(vec![file_metadata]);
        let response = client.create_file(request_stream).await;

        assert!(response.is_ok());

        let response = response.unwrap().into_inner();

        assert_eq!(response.name, "my_file");
        assert_eq!(response.r#type, "text/plain");
        assert_eq!(response.size, file_content.len() as u32);

        let mut read_dir = fs::read_dir(tmp_dir.path()).await.unwrap();

        let entry = read_dir.next_entry().await.unwrap();

        assert!(entry.is_some());
    }
}
