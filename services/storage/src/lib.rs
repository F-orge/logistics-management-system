use std::path::{Path, PathBuf};

use futures::StreamExt;
use sqlx::{types::Uuid, Acquire, PgPool, Pool, Postgres};
use tokio::{fs, sync::mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate_proto::storage::{
    file_metadata_request,
    storage_service_server::{StorageService as GRPCStorageService, StorageServiceServer},
    CreateFileRequest, DeleteFileRequest, DownloadFileRequest, FileChunk, FileMetadata,
    FileMetadataRequest,
};

pub struct StorageService {
    db: PgPool,
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
    type ListOwnedFilesStream = ReceiverStream<Result<FileMetadata, Status>>;
    type ListSharedFilesStream = ReceiverStream<Result<FileMetadata, Status>>;

    async fn create_file(
        &self,
        request: tonic::Request<tonic::Streaming<CreateFileRequest>>,
    ) -> std::result::Result<tonic::Response<FileMetadata>, tonic::Status> {
        let mut conn = match self.db.acquire().await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to aquire connection"));
            }
        };

        let mut trx = match conn.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to start transaction"));
            }
        };

        if let Err(err) = crate_utils::db::setup_db(&mut trx, request.metadata()).await {
            tracing::error!("{}", err);
            return Err(Status::internal("Unable to setup database"));
        }

        // refer to this documentation. https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        // under client side streaming section.
        let mut stream = request.into_inner();

        let mut chunks = Vec::new();

        let mut metadata = FileMetadata::default();

        let mut has_inserted_to_db = false;

        while let Some(chunk) = stream.next().await {
            let file_request = match chunk {
                Ok(file_request) => file_request,
                Err(err) => return Err(err),
            };

            let create_metadata = match file_request.metadata {
                Some(metadata) => metadata,
                None => return Err(Status::invalid_argument("Cannot get file metadata")),
            };

            if !has_inserted_to_db {
                let record = match sqlx::query!(
                        r#"insert into "storage"."file" (name, type, size) values ($1, $2, $3) returning id"#,
                        create_metadata.name,
                        create_metadata.r#type,
                        create_metadata.size as i32
                    )
                    .fetch_one(&mut *trx)
                    .await {
                        Ok(record) => record,
                        Err(err) => {
                            println!("{}",err);
                            return Err(Status::internal("Unable to insert file"))
                        }
                    };
                has_inserted_to_db = true;
                let record = sqlx::query!("select * from storage.file where id = $1", record.id)
                    .fetch_one(&mut *trx)
                    .await
                    .unwrap();

                metadata = FileMetadata {
                    id: record.id.to_string(),
                    name: record.name,
                    r#type: record.r#type,
                    size: record.size as u32,
                    is_public: record.is_public,
                    owner_id: record.owner_id.unwrap().to_string(),
                };
            }
            match file_request.chunk {
                Some(bytes) => chunks.push(bytes.chunk),
                None => return Err(Status::data_loss("Cannot get chunk")),
            }
        }

        let file_contents = chunks.into_iter().flatten().collect::<Vec<u8>>();

        let file_path = self.directory.join(format!("{}", metadata.id));

        // check if file chunks have the same size as metadata.size
        if file_contents.len() != metadata.size as usize {
            println!("{} {} {}", metadata.id, file_contents.len(), metadata.size);
            return Err(Status::data_loss("Invalid file size"));
        }

        match fs::write(file_path, file_contents).await {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Cannot write file to the server"));
            }
        };

        if let Err(err) = trx.commit().await {
            tracing::error!("{}", err);
            return Err(Status::internal("Unable to commit transaction"));
        };

        Ok(Response::new(metadata))
    }

    async fn list_owned_files(
        &self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<Self::ListOwnedFilesStream>, tonic::Status> {
        unimplemented!()
    }

    async fn list_shared_files(
        &self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<Self::ListSharedFilesStream>, tonic::Status> {
        unimplemented!()
    }

    async fn share_file(
        &self,
        request: tonic::Request<crate_proto::storage::ShareFileRequest>,
    ) -> std::result::Result<tonic::Response<crate_proto::storage::FileMetadata>, tonic::Status>
    {
        unimplemented!()
    }

    async fn download_file(
        &self,
        request: tonic::Request<DownloadFileRequest>,
    ) -> std::result::Result<tonic::Response<Self::DownloadFileStream>, tonic::Status> {
        let mut conn = match self.db.acquire().await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to aquire connection"));
            }
        };

        let mut trx = match conn.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to start transaction"));
            }
        };

        if let Err(err) = crate_utils::db::setup_db(&mut trx, request.metadata()).await {
            tracing::error!("{}", err);
            return Err(Status::internal("Unable to setup database"));
        }

        let file_id = match request.into_inner().id.parse::<Uuid>() {
            Ok(file_id) => file_id,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::invalid_argument("Cannot parse Uuid"));
            }
        };

        if let Err(err) = sqlx::query!("select * from storage.file where id = $1", file_id)
            .fetch_one(&mut *trx)
            .await
        {
            tracing::error!("{}", err);
            return Err(Status::not_found("File not found"));
        };

        let file_path = self.directory.join(format!("{}", file_id));

        if !file_path.exists() {
            let delete_request = DeleteFileRequest {
                id: file_id.to_string(),
            };
            match self.delete_file(Request::new(delete_request)).await {
                Ok(_) => return Err(Status::not_found("File not found on disk")),
                Err(err) => {
                    tracing::error!("{}", err);
                    return Err(Status::internal(
                        "An error occured when deleting file in the database",
                    ));
                }
            };
        }

        let (tx, rx) = mpsc::channel(32);

        let chunk_size = 1024 * 64; // 64KB chunks

        tokio::spawn(async move {
            let file_contents = match fs::read(file_path).await {
                Ok(contents) => contents,
                Err(err) => {
                    tracing::error!("{}", err);
                    let _ = tx.send(Err(Status::internal("Failed to read file"))).await;
                    return;
                }
            };

            for chunk in file_contents.chunks(chunk_size) {
                if let Err(err) = tx
                    .send(Ok(FileChunk {
                        chunk: chunk.to_vec(),
                    }))
                    .await
                {
                    tracing::error!("{}", err);
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_file_metadata(
        &self,
        request: tonic::Request<FileMetadataRequest>,
    ) -> std::result::Result<tonic::Response<FileMetadata>, tonic::Status> {
        let mut conn = match self.db.acquire().await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to aquire connection"));
            }
        };

        let mut trx = match conn.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to start transaction"));
            }
        };

        if let Err(err) = crate_utils::db::setup_db(&mut trx, request.metadata()).await {
            tracing::error!("{}", err);
            return Err(Status::internal("Unable to setup database"));
        }

        let payload = request.into_inner();

        let metadata = match payload.request {
            Some(file_metadata_request::Request::Id(id)) => {
                let file_id = match Uuid::parse_str(&id) {
                    Ok(file_id) => file_id,
                    Err(err) => {
                        tracing::error!("{}", err);
                        return Err(Status::invalid_argument("Invalid UUID format"));
                    }
                };

                let record =
                    match sqlx::query!(r#"select * from storage.file where id = $1"#, file_id)
                        .fetch_one(&mut *trx)
                        .await
                    {
                        Ok(record) => record,
                        Err(err) => {
                            tracing::error!("{}", err);
                            return Err(Status::not_found("File not found"));
                        }
                    };
                FileMetadata {
                    id: record.id.to_string(),
                    name: record.name,
                    r#type: record.r#type,
                    is_public: record.is_public,
                    owner_id: record.owner_id.unwrap().to_string(),
                    size: record.size as u32,
                }
            }
            Some(file_metadata_request::Request::Name(name)) => {
                let record =
                    sqlx::query!(r#"SELECT * FROM "storage"."file" WHERE name = $1"#, name)
                        .fetch_optional(&mut *trx)
                        .await
                        .map_err(|_| Status::internal("Database error"))?
                        .ok_or_else(|| Status::not_found("File not found"))?;
                FileMetadata {
                    id: record.id.to_string(),
                    name: record.name,
                    r#type: record.r#type,
                    is_public: record.is_public,
                    owner_id: record.owner_id.unwrap().to_string(),
                    size: record.size as u32,
                }
            }
            None => return Err(Status::invalid_argument("Missing request parameters")),
        };

        if let Err(err) = trx.commit().await {
            tracing::error!("{}", err);
            return Err(Status::internal("Unable to commit file to database"));
        };

        Ok(Response::new(metadata))
    }

    async fn delete_file(
        &self,
        request: tonic::Request<DeleteFileRequest>,
    ) -> Result<Response<()>, Status> {
        let mut conn = match self.db.acquire().await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to aquire connection"));
            }
        };

        let mut trx = match conn.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to start transaction"));
            }
        };

        if let Err(err) = crate_utils::db::setup_db(&mut trx, request.metadata()).await {
            tracing::error!("{}", err);
            return Err(Status::internal("Unable to setup database"));
        }

        let file_id = match request.into_inner().id.parse::<Uuid>() {
            Ok(uuid) => uuid,
            Err(err) => {
                tracing::error!("{}", err);
                return Err(Status::invalid_argument("Unable to parse Uuid"));
            }
        };

        if let Err(err) = sqlx::query!(r#"delete from storage.file where id = $1"#, file_id)
            .execute(&mut *trx)
            .await
        {
            tracing::error!("{}", err);

            if let Err(err) = trx.rollback().await {
                tracing::error!("{}", err);
                return Err(Status::internal("Unable to rollback delete operation"));
            }

            return Err(Status::not_found("Failed to delete file from database"));
        }

        if let Err(err) = trx.commit().await {
            tracing::error!("{}", err);
            return Err(Status::internal("Unable to commit delete operation"));
        }

        let file_path = self.directory.join(format!("{}", file_id));

        // delete from filesystem
        if let Err(err) = fs::remove_file(&file_path).await {
            tracing::error!("{}", err);
            return Err(Status::internal("Failed to delete file from disk"));
        }

        Ok(Response::new(()))
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use futures::TryStreamExt;
    use tempdir::TempDir;
    use tonic::{
        metadata::{MetadataMap, MetadataValue},
        service::interceptor::InterceptedService,
        transport::{Channel, Server},
        Request,
    };

    use crate_proto::{
        auth::{self, auth_service_client::AuthServiceClient, AuthBasicLoginRequest},
        storage::{storage_service_client::StorageServiceClient, CreateFileRequest},
    };

    use service_authentication::AuthService;

    use crate_utils::test::start_server;

    use super::*;

    async fn create_dummy_user(db: &Pool<Postgres>) {
        let mut trx = match db.begin().await {
            Ok(trx) => trx,
            Err(err) => {
                panic!("{}", err);
            }
        };

        if let Err(err) = crate_utils::db::setup_db(&mut trx, &MetadataMap::new()).await {
            panic!("{}", err);
        }

        if let Err(err) = sqlx::query("insert into auth.basic_user(email,password) values ($1,$2)")
            .bind("sample@email.com")
            .bind("Randompassword1!")
            .execute(&mut *trx)
            .await
        {
            panic!("{}", err);
        }

        if let Err(err) = trx.commit().await {
            panic!("{}", err);
        }
    }

    async fn setup_actor(db: &Pool<Postgres>) -> Result<auth::AuthResponse, tonic::Status> {
        create_dummy_user(db).await;

        let (_, channel) = start_server(Server::builder().add_service(AuthService::new(db))).await;

        let mut client = AuthServiceClient::new(channel);

        let response = client
            .basic_login(AuthBasicLoginRequest {
                email: "sample@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await;

        match response {
            Ok(response) => Ok(response.into_inner()),
            Err(err) => Err(err),
        }
    }

    async fn setup_test_client(
        db: &Pool<Postgres>,
        auth_response: auth::AuthResponse,
    ) -> (
        TempDir,
        StorageServiceClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    ) {
        let tmp_dir = TempDir::new("temp_storage").unwrap();

        let (_, channel) =
            start_server(Server::builder().add_service(StorageService::new(db, tmp_dir.path())))
                .await;

        let token: MetadataValue<_> = auth_response.access_token.parse().unwrap();

        let client = StorageServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            // NOTE: for metadata insertion and retrieval only use lowercase keys because inserting it will cause a panic.
            // see this bug post: https://github.com/hyperium/tonic/issues/1782
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        });
        (tmp_dir, client)
    }

    async fn create_test_file(
        client: &mut StorageServiceClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    ) -> FileMetadata {
        let file_content = b"Test file content";

        let request = CreateFileRequest {
            metadata: Some(crate_proto::storage::CreateFileMetadataRequest {
                name: "test_file.txt".into(),
                r#type: "text/plain".into(),
                is_public: false,
                size: file_content.len() as u32,
            }),
            chunk: Some(FileChunk {
                chunk: file_content.to_vec(),
            }),
        };

        let request_stream = tokio_stream::iter(vec![request]);
        client
            .create_file(request_stream)
            .await
            .unwrap()
            .into_inner()
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_storage_create_file(db: Pool<Postgres>) {
        std::env::set_var("RUST_POSTGRES_JWT_SECRET", "randomsecret");
        std::env::set_var("RUST_POSTGRES_JWT_AUDIENCE", "management.com");
        std::env::set_var("RUST_POSTGRES_JWT_ISSUER", "web");
        std::env::set_var("RUST_POSTGRES_JWT_EXPIRY", "3600");

        let actor = setup_actor(&db).await.unwrap();

        let (tmp_dir, mut client) = setup_test_client(&db, actor).await;

        // send one chunk to the ba`ckend
        let file_content = b"HELLO MY NAME IS JOHN DOE. i am a file!!! :3";

        let file_metadata = CreateFileRequest {
            metadata: Some(crate_proto::storage::CreateFileMetadataRequest {
                name: "test_file.txt".into(),
                r#type: "text/plain".into(),
                is_public: false,
                size: file_content.len() as u32,
            }),
            chunk: Some(FileChunk {
                chunk: file_content.to_vec(),
            }),
        };

        let request_stream = tokio_stream::iter(vec![file_metadata]);
        let response = client.create_file(request_stream).await;

        assert!(response.is_ok(), "{:#?}", response.err());

        let response = response.unwrap().into_inner();

        assert_eq!(response.name, "test_file.txt");
        assert_eq!(response.r#type, "text/plain");
        assert_eq!(response.size, file_content.len() as u32);

        let mut read_dir = fs::read_dir(tmp_dir.path()).await.unwrap();

        let entry = read_dir.next_entry().await.unwrap();

        // check if we really store it in the file system.
        assert!(entry.is_some());
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_storage_download_file(db: Pool<Postgres>) {
        std::env::set_var("RUST_POSTGRES_JWT_SECRET", "randomsecret");
        std::env::set_var("RUST_POSTGRES_JWT_AUDIENCE", "management.com");
        std::env::set_var("RUST_POSTGRES_JWT_ISSUER", "web");
        std::env::set_var("RUST_POSTGRES_JWT_EXPIRY", "3600");

        let actor = setup_actor(&db).await.unwrap();

        let (_temp_dir, mut client) = setup_test_client(&db, actor.clone()).await;

        // create a file
        let metadata = create_test_file(&mut client).await;

        let mut metadata_request = Request::new(DownloadFileRequest { id: metadata.id });

        metadata_request
            .metadata_mut()
            .append("authorization", actor.access_token.parse().unwrap());

        // download file
        let response = client.download_file(metadata_request).await.unwrap();

        let chunks: Vec<FileChunk> = response.into_inner().try_collect().await.unwrap();

        let content = chunks
            .into_iter()
            .flat_map(|chunk| chunk.chunk)
            .collect::<Vec<u8>>();

        assert_eq!(content, b"Test file content");
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_storage_get_file_metadata(db: Pool<Postgres>) {
        std::env::set_var("RUST_POSTGRES_JWT_SECRET", "randomsecret");
        std::env::set_var("RUST_POSTGRES_JWT_AUDIENCE", "management.com");
        std::env::set_var("RUST_POSTGRES_JWT_ISSUER", "web");
        std::env::set_var("RUST_POSTGRES_JWT_EXPIRY", "3600");

        let actor = setup_actor(&db).await.unwrap();

        let (_temp_dir, mut client) = setup_test_client(&db, actor.clone()).await;

        // create a file
        let metadata = create_test_file(&mut client).await;

        let mut metadata_request = Request::new(FileMetadataRequest {
            request: Some(file_metadata_request::Request::Id(metadata.clone().id)),
        });

        metadata_request
            .metadata_mut()
            .append("authorization", actor.access_token.parse().unwrap());

        // download file
        let response = client.get_file_metadata(metadata_request).await;

        assert!(response.is_ok());

        let response = response.unwrap().into_inner();

        assert_eq!(metadata.id, response.id);
        assert_eq!(metadata.name, response.name);
        assert_eq!(metadata.r#type, response.r#type);
        assert_eq!(metadata.size, response.size);
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_delete_file(db: Pool<Postgres>) {
        std::env::set_var("RUST_POSTGRES_JWT_SECRET", "randomsecret");
        std::env::set_var("RUST_POSTGRES_JWT_AUDIENCE", "management.com");
        std::env::set_var("RUST_POSTGRES_JWT_ISSUER", "web");
        std::env::set_var("RUST_POSTGRES_JWT_EXPIRY", "3600");

        let actor = setup_actor(&db).await.unwrap();

        let (_temp_dir, mut client) = setup_test_client(&db, actor.clone()).await;

        // create a file
        let metadata = create_test_file(&mut client).await;

        // Delete the file
        let file_id = metadata.id.clone();

        let response = client
            .delete_file(Request::new(DeleteFileRequest {
                id: file_id.clone(),
            }))
            .await;

        assert!(response.is_ok(), "{:#?}", response.err());

        // try to get the deleted file - should return not found
        let get_response = client
            .get_file_metadata(Request::new(FileMetadataRequest {
                request: Some(file_metadata_request::Request::Id(file_id)),
            }))
            .await;

        assert!(
            get_response.is_err(),
            "File should not exist after deletion {:#?}",
            get_response.ok()
        );
        assert_eq!(get_response.unwrap_err().code(), tonic::Code::NotFound);
    }
}
