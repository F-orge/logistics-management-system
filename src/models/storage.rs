use std::{
    path::{Path, PathBuf},
    pin::Pin,
};

use futures::{Stream, StreamExt};
use sqlx::{types::Uuid, Pool, Postgres};
use tokio::{fs, sync::mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

use crate::models::_proto::{
    self,
    storage::{
        storage_service_server::{StorageService as GRPCStorageService, StorageServiceServer},
        FileChunk, FileExistsResponse, FileMetadata,
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

    async fn get_file_by_id(&self, id: &str) -> Result<FileMetadata, Status> {
        let uuid =
            Uuid::parse_str(id).map_err(|_| Status::invalid_argument("Invalid UUID format"))?;

        let record = sqlx::query!(r#"SELECT * FROM storage.file WHERE id = $1"#, uuid)
            .fetch_optional(&self.db)
            .await
            .map_err(|_| Status::internal("Database error"))?
            .ok_or_else(|| Status::not_found("File not found"))?;

        Ok(FileMetadata {
            id: Some(record.id.to_string()),
            name: record.name,
            r#type: record.r#type,
            size: record.size as u32,
        })
    }
}

#[tonic::async_trait]
impl GRPCStorageService for StorageService {
    type DownloadFileStream = ReceiverStream<Result<FileChunk, Status>>;

    async fn create_file(
        &self,
        request: tonic::Request<tonic::Streaming<_proto::storage::CreateFileRequest>>,
    ) -> Result<Response<FileMetadata>, Status> {
        let mut stream = request.into_inner();
        let mut chunks = Vec::new();
        let mut metadata = FileMetadata::default();
        let mut total_size = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| Status::internal(e.to_string()))?;

            if metadata.name.is_empty() {
                metadata = chunk
                    .metadata
                    .ok_or_else(|| Status::invalid_argument("First chunk must contain metadata"))?;
            }

            if let Some(chunk_data) = chunk.chunk {
                total_size += chunk_data.chunk.len();
                chunks.push(chunk_data.chunk);
            }
        }

        // validate total size matches metadata
        if total_size != metadata.size as usize {
            return Err(Status::invalid_argument("Size mismatch"));
        }

        let file_id = metadata.id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let file_path = self
            .directory
            .join(format!("{}-{}", file_id, metadata.name));

        // write file to disk
        let file_contents: Vec<u8> = chunks.into_iter().flatten().collect();
        fs::write(&file_path, &file_contents)
            .await
            .map_err(|_| Status::internal("Failed to write file"))?;

        // save to database
        let record = sqlx::query!(
            r#"
            INSERT INTO "storage"."file" (id, name, type, size)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            Uuid::parse_str(&file_id).map_err(|_| Status::internal("Invalid UUID"))?,
            metadata.name,
            metadata.r#type,
            metadata.size as i64
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| Status::internal("Database error"))?;

        Ok(Response::new(FileMetadata {
            id: Some(record.id.to_string()),
            name: record.name,
            r#type: record.r#type,
            size: record.size as u32,
        }))
    }

    async fn download_file(
        &self,
        request: tonic::Request<_proto::storage::DownloadFileRequest>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        let file_id = request.into_inner().id;
        let metadata = self.get_file_by_id(&file_id).await?;

        let file_path = self
            .directory
            .join(format!("{}-{}", file_id, metadata.name));

        if !file_path.exists() {
            return Err(Status::not_found("File not found on disk"));
        }

        let (tx, rx) = mpsc::channel(32);
        let chunk_size = 1024 * 64; // 64KB chunks

        tokio::spawn(async move {
            let file_contents = match fs::read(file_path).await {
                Ok(contents) => contents,
                Err(_) => {
                    let _ = tx.send(Err(Status::internal("Failed to read file"))).await;
                    return;
                }
            };

            for chunk in file_contents.chunks(chunk_size) {
                if tx
                    .send(Ok(FileChunk {
                        chunk: chunk.to_vec(),
                    }))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_file_metadata(
        &self,
        request: tonic::Request<_proto::storage::FileMetadataRequest>,
    ) -> Result<Response<FileMetadata>, Status> {
        let request = request.into_inner();

        let metadata = match request.request {
            Some(_proto::storage::file_metadata_request::Request::Id(id)) => {
                self.get_file_by_id(&id).await?
            }
            Some(_proto::storage::file_metadata_request::Request::Name(name)) => {
                let record = sqlx::query!(r#"SELECT * FROM storage.file WHERE name = $1"#, name)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(|_| Status::internal("Database error"))?
                    .ok_or_else(|| Status::not_found("File not found"))?;

                FileMetadata {
                    id: Some(record.id.to_string()),
                    name: record.name,
                    r#type: record.r#type,
                    size: record.size as u32,
                }
            }
            None => return Err(Status::invalid_argument("Missing request parameters")),
        };

        Ok(Response::new(metadata))
    }

    async fn delete_file(
        &self,
        request: tonic::Request<_proto::storage::DeleteFileRequest>,
    ) -> Result<Response<()>, Status> {
        let file_id = request.into_inner().id;
        let metadata = self.get_file_by_id(&file_id).await?;

        let file_path = self
            .directory
            .join(format!("{}-{}", file_id, metadata.name));

        //  delete from filesystem
        if let Err(_) = fs::remove_file(&file_path).await {
            return Err(Status::internal("Failed to delete file from disk"));
        }

        // delete from database
        let uuid = Uuid::parse_str(&file_id)
            .map_err(|_| Status::invalid_argument("Invalid UUID format"))?;

        match sqlx::query!(r#"DELETE FROM storage.file WHERE id = $1"#, uuid)
            .execute(&self.db)
            .await
        {
            Ok(_) => Ok(Response::new(())),
            Err(_) => Err(Status::internal("Failed to delete file from database")),
        }
    }

    async fn file_exists(
        &self,
        request: tonic::Request<_proto::storage::FileMetadataRequest>,
    ) -> Result<Response<FileExistsResponse>, Status> {
        let request = request.into_inner();

        let exists = match request.request {
            Some(_proto::storage::file_metadata_request::Request::Id(id)) => {
                let uuid = match Uuid::parse_str(&id) {
                    Ok(uuid) => uuid,
                    Err(_) => return Err(Status::invalid_argument("Invalid UUID format")),
                };

                let record = sqlx::query!(
                    r#"
                    INSERT INTO storage.file (id, name, "type", size)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                    "#,
                    Uuid::parse_str(&file_id).map_err(|_| Status::internal("Invalid UUID"))?,
                    metadata.name,
                    metadata.r#type,
                    metadata.size as i64
                );
                result.exists
            }
            Some(_proto::storage::file_metadata_request::Request::Name(name)) => {
                let record = sqlx::query!(
                    r#"
                    INSERT INTO storage.file (id, name, "type", size)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                    "#,
                    Uuid::parse_str(&file_id).map_err(|_| Status::internal("Invalid UUID"))?,
                    metadata.name,
                    metadata.r#type,
                    metadata.size as i64
                );
            }
            None => return Err(Status::invalid_argument("Missing request parameters")),
        };

        Ok(Response::new(FileExistsResponse { exists }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::_proto::storage::{
        file_metadata_request, storage_service_client::StorageServiceClient, CreateFileRequest,
        DeleteFileRequest, DownloadFileRequest, FileMetadataRequest,
    };
    use crate::utils::test::start_server;
    use futures::TryStreamExt;
    use tempdir::TempDir;
    use tonic::{transport::Server, Request};

    async fn setup_test_client(
        db: &Pool<Postgres>,
    ) -> (TempDir, StorageServiceClient<tonic::transport::Channel>) {
        let tmp_dir = TempDir::new("temp_storage").unwrap();
        let (_, channel) =
            start_server(Server::builder().add_service(StorageService::new(db, tmp_dir.path())))
                .await;
        let client = StorageServiceClient::new(channel);
        (tmp_dir, client)
    }

    async fn create_test_file(
        client: &mut StorageServiceClient<tonic::transport::Channel>,
    ) -> FileMetadata {
        let file_content = b"Test file content";
        let request = CreateFileRequest {
            metadata: Some(FileMetadata {
                id: None,
                name: "test_file.txt".into(),
                r#type: "text/plain".into(),
                size: file_content.len() as u32,
            }),
            chunk: Some(FileChunk {
                chunk: file_content.to_vec(),
            }),
            total_chunks: 1,
            chunk_number: 1,
        };

        let request_stream = tokio_stream::iter(vec![request]);
        client
            .create_file(request_stream)
            .await
            .unwrap()
            .into_inner()
    }

    #[sqlx::test]
    async fn test_download_file(db: Pool<Postgres>) {
        let (_tmp_dir, mut client) = setup_test_client(&db).await;

        // create a file
        let metadata = create_test_file(&mut client).await;

        // download file
        let response = client
            .download_file(Request::new(DownloadFileRequest {
                id: metadata.id.unwrap(),
            }))
            .await
            .unwrap();

        let chunks: Vec<FileChunk> = response.into_inner().try_collect().await.unwrap();

        let content = chunks
            .into_iter()
            .flat_map(|chunk| chunk.chunk)
            .collect::<Vec<u8>>();

        assert_eq!(content, b"Test file content");
    }

    #[sqlx::test]
    async fn test_get_file_metadata(db: Pool<Postgres>) {
        let (_tmp_dir, mut client) = setup_test_client(&db).await;

        // create a file first
        let created_metadata = create_test_file(&mut client).await;

        // test get by ID
        let response = client
            .get_file_metadata(Request::new(FileMetadataRequest {
                request: Some(file_metadata_request::Request::Id(
                    created_metadata.id.clone().unwrap(),
                )),
            }))
            .await
            .unwrap();

        let metadata = response.into_inner();
        assert_eq!(metadata.name, "test_file.txt");
        assert_eq!(metadata.r#type, "text/plain");

        // test get by name
        let response = client
            .get_file_metadata(Request::new(FileMetadataRequest {
                request: Some(file_metadata_request::Request::Name(
                    "test_file.txt".to_string(),
                )),
            }))
            .await
            .unwrap();

        let metadata = response.into_inner();
        assert_eq!(metadata.id, created_metadata.id);
    }

    #[sqlx::test]
    async fn test_delete_file(db: Pool<Postgres>) {
        let (_tmp_dir, mut client) = setup_test_client(&db).await;

        // create a file first
        let metadata = create_test_file(&mut client).await;

        // delete the file
        client
            .delete_file(Request::new(DeleteFileRequest {
                id: metadata.id.unwrap(),
            }))
            .await
            .unwrap();

        // verify file doesn't exist
        let response = client
            .file_exists(Request::new(FileMetadataRequest {
                request: Some(file_metadata_request::Request::Name(
                    "test_file.txt".to_string(),
                )),
            }))
            .await
            .unwrap();

        assert!(!response.into_inner().exists);
    }

    #[sqlx::test]
    async fn test_file_exists(db: Pool<Postgres>) {
        let (_tmp_dir, mut client) = setup_test_client(&db).await;

        // test non-existent file
        let response = client
            .file_exists(Request::new(FileMetadataRequest {
                request: Some(file_metadata_request::Request::Name(
                    "nonexistent.txt".to_string(),
                )),
            }))
            .await
            .unwrap();

        assert!(!response.into_inner().exists);

        // create a file
        let metadata = create_test_file(&mut client).await;

        // test existing file by ID
        let response = client
            .file_exists(Request::new(FileMetadataRequest {
                request: Some(file_metadata_request::Request::Id(metadata.id.unwrap())),
            }))
            .await
            .unwrap();

        assert!(response.into_inner().exists);
    }
}
