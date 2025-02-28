use std::path::{Path, PathBuf};

use futures::{StreamExt, TryStreamExt};
use hmac::Hmac;
use lib_core::error::Error;
use lib_entity::{file, file_access, prelude::*};
use lib_security::get_jwt_claim;
use sea_orm::{
    prelude::Expr, ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Condition,
    DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set, TransactionTrait,
};
use sha2::Sha256;
use sqlx::types::Uuid;
use tokio::fs;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use lib_proto::storage::{
    storage_service_server::{StorageService as GRPCStorageService, StorageServiceServer},
    CreateFileRequest, DeleteFileRequest, DownloadFileRequest, FileChunk, FileMetadata,
    FileMetadataRequest,
};

pub struct StorageService {
    db: DatabaseConnection,
    directory: PathBuf,
    encryption_key: Hmac<Sha256>,
}

impl StorageService {
    pub fn new(
        db: &DatabaseConnection,
        directory: &Path,
        encryption_key: Hmac<Sha256>,
    ) -> StorageServiceServer<Self> {
        StorageServiceServer::new(Self {
            db: db.clone(),
            directory: directory.to_path_buf(),
            encryption_key,
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
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let claims = lib_security::get_jwt_claim(&request.metadata(), &self.encryption_key)?;

        // refer to this documentation. https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
        // under client side streaming section.
        let mut stream = request.into_inner();

        let mut chunks = Vec::new();

        let mut metadata = FileMetadata::default();

        let mut has_inserted_to_db = false;

        while let Some(chunk) = stream.next().await {
            let file_request = chunk?;

            let create_metadata = match file_request.metadata {
                Some(metadata) => metadata,
                None => return Err(Status::invalid_argument("Cannot get file metadata")),
            };

            if !has_inserted_to_db {
                let mut file = file::ActiveModel::new();

                file.name = Set(create_metadata.name);
                file.is_public = Set(create_metadata.is_public);
                file.owner_id = Set(claims.subject);
                file.r#type = Set(create_metadata.r#type);
                file.size = Set(create_metadata.size as i32);

                let file = file.insert(&trx).await.map_err(Error::SeaOrm)?;

                let mut file_access = file_access::ActiveModel::new();

                file_access.file_id = Set(file.id);
                file_access.user_id = Set(claims.subject);

                let _ = file_access.insert(&trx).await.map_err(Error::SeaOrm)?;

                has_inserted_to_db = true;

                metadata = file.into();
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
            tracing::debug!("{} {} {}", metadata.id, file_contents.len(), metadata.size);
            return Err(Status::data_loss("Invalid file size"));
        }

        fs::write(file_path, file_contents)
            .await
            .map_err(lib_core::error::Error::Io)?;

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(metadata))
    }

    async fn list_owned_files(
        &self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<Self::ListOwnedFilesStream>, tonic::Status> {
        let claims = get_jwt_claim(&request.metadata(), &self.encryption_key)?;

        let mut user_files = File::find()
            .filter(file::Column::OwnerId.eq(claims.subject))
            .stream(&self.db)
            .await
            .map_err(Error::SeaOrm)?;

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<FileMetadata, Status>>(1024 * 64);

        while let Some(row) = user_files.try_next().await.map_err(Error::SeaOrm)? {
            let item: file::Model = row.into();
            tx.send(Ok(item.into()))
                .await
                .map_err(|err| Error::Custom(Box::new(err)))?;
        }
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn list_shared_files(
        &self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<Self::ListSharedFilesStream>, tonic::Status> {
        let claims = get_jwt_claim(&request.metadata(), &self.encryption_key)?;

        let mut shared_files =
            File::find()
                .inner_join(FileAccess)
                .filter(file::Column::OwnerId.ne(claims.subject))
                .filter(Condition::any().add(file::Column::IsPublic.eq(true)).add(
                    file_access::Column::UserId.eq(claims.subject).and(
                        Expr::col(file_access::Column::FileId).eq(Expr::col(file::Column::Id)),
                    ),
                ))
                .stream(&self.db)
                .await
                .map_err(Error::SeaOrm)?;

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<FileMetadata, Status>>(1024 * 64);

        while let Some(row) = shared_files.try_next().await.map_err(Error::SeaOrm)? {
            let item: file::Model = row.into();
            tx.send(Ok(item.into()))
                .await
                .map_err(|err| Error::Custom(Box::new(err)))?;
        }
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn share_file(
        &self,
        request: tonic::Request<lib_proto::storage::ShareFileRequest>,
    ) -> std::result::Result<Response<()>, tonic::Status> {
        let claims = get_jwt_claim(&request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let payload = request.into_inner();

        // TODO make sure that the user owned the file
        let file = File::find()
            .filter(file::Column::OwnerId.eq(claims.subject))
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        // check if we have the right properties.
        if payload.user_ids.len() == 0 && payload.share_option.is_none() {
            return Err(Status::invalid_argument(
                "user_ids and share options are both empty",
            ));
        }

        if payload.user_ids.len() > 0 && payload.share_option.is_some() {
            return Err(Status::invalid_argument(
                "user_ids and share options are both present",
            ));
        }

        let user_ids = payload
            .user_ids
            .into_iter()
            .map(|v| v.parse::<Uuid>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(Error::Uuid)?;

        if let Some(share_option) = payload.share_option {
            match share_option {
                lib_proto::share_file_request::ShareOption::IsPublic(is_public) => {
                    let mut file = file.clone().into_active_model();

                    file.is_public = Set(is_public);

                    let _ = file.update(&trx).await.map_err(Error::SeaOrm)?;

                    trx.commit().await.map_err(Error::SeaOrm)?;

                    return Ok(Response::new(()));
                }
            }
        }

        for user_id in user_ids {
            let mut insert_stmt = file_access::ActiveModel::new();

            insert_stmt.file_id = Set(file.id);
            insert_stmt.user_id = Set(user_id);

            insert_stmt.insert(&trx).await.map_err(Error::SeaOrm)?;
        }

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn download_file(
        &self,
        request: tonic::Request<DownloadFileRequest>,
    ) -> std::result::Result<tonic::Response<Self::DownloadFileStream>, tonic::Status> {
        let claims = get_jwt_claim(&request.metadata(), &self.encryption_key)?;

        let file_id = request
            .into_inner()
            .id
            .parse::<Uuid>()
            .map_err(Error::Uuid)?;

        // TODO: make sure that the file is either owned by the user, is_public = true, or shared by other users
        let file = File::find()
            .inner_join(FileAccess)
            .filter(file::Column::Id.eq(file_id))
            .filter(
                Condition::any()
                    .add(file::Column::OwnerId.eq(claims.subject))
                    .add(file::Column::IsPublic.eq(true))
                    .add(
                        file_access::Column::UserId
                            .eq(claims.subject)
                            .and(file_access::Column::FileId.eq(file_id)),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?;

        let file_path = self.directory.join(format!("{}", file.id));

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

        let chunk_size = 1024 * 64; // 64KB chunks

        let contents = fs::read(file_path)
            .await
            .map_err(lib_core::error::Error::Io)?;

        let file_chunks = contents
            .chunks(chunk_size)
            .map(|v| FileChunk { chunk: v.to_vec() })
            .collect::<Vec<_>>();

        Ok(Response::new(
            lib_core::streaming::stream_iterator(file_chunks.into_iter()).await,
        ))
    }

    async fn get_file_metadata(
        &self,
        request: tonic::Request<FileMetadataRequest>,
    ) -> std::result::Result<tonic::Response<FileMetadata>, tonic::Status> {
        let claims = get_jwt_claim(&request.metadata(), &self.encryption_key)?;

        let payload = request.into_inner();

        let file_id = payload.id.parse::<Uuid>().map_err(Error::Uuid)?;

        let file = File::find()
            .inner_join(FileAccess)
            .filter(file::Column::Id.eq(file_id))
            .filter(
                Condition::any()
                    .add(file::Column::OwnerId.eq(claims.subject))
                    .add(file::Column::IsPublic.eq(true))
                    .add(
                        file_access::Column::UserId
                            .eq(claims.subject)
                            .and(file_access::Column::FileId.eq(file_id)),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into();

        Ok(Response::new(file))
    }

    async fn delete_file(
        &self,
        request: tonic::Request<DeleteFileRequest>,
    ) -> Result<Response<()>, Status> {
        let claims = get_jwt_claim(&request.metadata(), &self.encryption_key)?;

        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let payload = request.into_inner();

        let file_id = payload.id.parse::<Uuid>().map_err(Error::Uuid)?;

        let file = File::find()
            .filter(file::Column::Id.eq(file_id))
            .filter(file::Column::OwnerId.eq(claims.subject))
            .one(&trx)
            .await
            .map_err(Error::SeaOrm)?
            .ok_or(Error::RowNotFound)?
            .into_active_model();

        let result = file.delete(&trx).await.map_err(Error::SeaOrm)?;

        if result.rows_affected == 0 {
            return Err(Status::internal("Unable to delete file from database"));
        }

        trx.commit().await.map_err(Error::SeaOrm)?;

        let file_path = self.directory.join(format!("{}", file_id));

        // delete from filesystem
        fs::remove_file(&file_path)
            .await
            .map_err(lib_core::error::Error::Io)?;

        Ok(Response::new(()))
    }
}

#[cfg(test)]
mod test {
    use sea_orm::EntityTrait;
    use std::str::FromStr;

    use futures::TryStreamExt;
    use hmac::{Hmac, Mac};
    use lib_core::test::start_server;
    use lib_entity::sea_orm_active_enums::AuthType;
    use lib_entity::{file, users};
    use lib_proto::auth_service_client::AuthServiceClient;
    use lib_proto::storage_service_client::StorageServiceClient;
    use lib_proto::{
        AuthBasicLoginRequest, CreateFileRequest, DeleteFileRequest, DownloadFileRequest,
        FileChunk, ShareFileRequest,
    };
    use sea_orm::{ActiveModelBehavior, ActiveModelTrait, Database, Set};
    use service_authentication::AuthService;
    use sqlx::ConnectOptions;
    use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, Postgres};
    use tonic::metadata::MetadataValue;
    use tonic::transport::Server;
    use tonic::{Request, Status};

    use crate::StorageService;

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_create_file(
        _: PoolOptions<Postgres>,
        conn_options: PgConnectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -- setup

        let db = Database::connect(conn_options.to_url_lossy()).await?;

        let mut user = users::ActiveModel::new();

        user.email = Set("sample@email.com".into());
        user.password = Set("Randompassword1!".into());
        user.auth_type = Set(AuthType::BasicAuth);

        let _ = user.insert(&db).await?;

        let temp_dir = tempdir::TempDir::new("sample")?;

        let key = Hmac::new_from_slice(b"random-encryptio-key")?;

        let (_, storage_channel) = start_server(
            Server::builder().add_service(StorageService::new(&db, temp_dir.path(), key.clone())),
        )
        .await;

        let (_, auth_channel) =
            start_server(Server::builder().add_service(AuthService::new(&db, key.clone()))).await;

        let mut auth_client = AuthServiceClient::new(auth_channel);

        let user_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let mut storage_client =
            StorageServiceClient::with_interceptor(storage_channel, |mut request: Request<()>| {
                let token = format!("Bearer {}", user_auth.access_token);
                request.metadata_mut().append(
                    "authorization",
                    MetadataValue::from_str(&token).map_err(|_| Status::internal("error"))?,
                );
                Ok(request)
            });

        // -- end setup

        // -- start execution

        let file_content = br#"
            This is a sample .txt file
        "#;

        let file_metadata = CreateFileRequest {
            metadata: Some(lib_proto::storage::CreateFileMetadataRequest {
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

        let response = storage_client.create_file(request_stream).await;

        // -- end execution

        // -- validation

        assert!(response.is_ok(), "{:?}", response.err());

        let directory = std::fs::read_dir(temp_dir.path())?.collect::<Vec<_>>();

        assert_eq!(directory.len(), 1);

        // -- end validation

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_list_owned_files(
        _: PoolOptions<Postgres>,
        conn_options: PgConnectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -- setup

        let db = Database::connect(conn_options.to_url_lossy()).await?;

        let mut user = users::ActiveModel::new();

        user.email = Set("sample@email.com".into());
        user.password = Set("Randompassword1!".into());
        user.auth_type = Set(AuthType::BasicAuth);

        let _ = user.insert(&db).await?;

        let temp_dir = tempdir::TempDir::new("sample")?;

        let key = Hmac::new_from_slice(b"random-encryptio-key")?;

        let (_, storage_channel) = start_server(
            Server::builder().add_service(StorageService::new(&db, temp_dir.path(), key.clone())),
        )
        .await;

        let (_, auth_channel) =
            start_server(Server::builder().add_service(AuthService::new(&db, key.clone()))).await;

        let mut auth_client = AuthServiceClient::new(auth_channel);

        let user_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let mut storage_client =
            StorageServiceClient::with_interceptor(storage_channel, |mut request: Request<()>| {
                let token = format!("Bearer {}", user_auth.access_token);
                request.metadata_mut().append(
                    "authorization",
                    MetadataValue::from_str(&token).map_err(|_| Status::internal("error"))?,
                );

                Ok(request)
            });

        let file_content = br#"
            This is a sample .txt file
        "#;

        let file_metadata = CreateFileRequest {
            metadata: Some(lib_proto::storage::CreateFileMetadataRequest {
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

        let _ = storage_client.create_file(request_stream).await;

        // -- end setup

        // -- start execution

        let response = storage_client
            .list_owned_files(())
            .await?
            .into_inner()
            .try_collect::<Vec<_>>()
            .await?;

        // -- end execution

        // -- validation

        assert_eq!(response.len(), 1);

        // -- end validation

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_share_file(
        _: PoolOptions<Postgres>,
        conn_options: PgConnectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -- setup

        let db = Database::connect(conn_options.to_url_lossy()).await?;

        let temp_dir = tempdir::TempDir::new("sample")?;

        let mut user_1 = users::ActiveModel::new();

        user_1.email = Set("sample@email.com".into());
        user_1.password = Set("Randompassword1!".into());
        user_1.auth_type = Set(AuthType::BasicAuth);

        let _ = user_1.insert(&db).await?;

        let mut user_2 = users::ActiveModel::new();

        user_2.email = Set("sample2@email.com".into());
        user_2.password = Set("Randompassword1!".into());
        user_2.auth_type = Set(AuthType::BasicAuth);

        let user_2 = user_2.insert(&db).await?;

        let key = Hmac::new_from_slice(b"random-encryptio-key")?;

        let (_, storage_channel) = start_server(
            Server::builder().add_service(StorageService::new(&db, temp_dir.path(), key.clone())),
        )
        .await;

        let (_, auth_channel) =
            start_server(Server::builder().add_service(AuthService::new(&db, key.clone()))).await;

        let mut auth_client = AuthServiceClient::new(auth_channel);

        let user_1_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let _ = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample2@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let mut storage_client = StorageServiceClient::new(storage_channel);

        let file_content = br#"
            This is a sample .txt file
        "#;

        let file_metadata = CreateFileRequest {
            metadata: Some(lib_proto::storage::CreateFileMetadataRequest {
                name: "test_file.txt".into(),
                r#type: "text/plain".into(),
                is_public: false,
                size: file_content.len() as u32,
            }),
            chunk: Some(FileChunk {
                chunk: file_content.to_vec(),
            }),
        };

        let mut request_stream = Request::new(tokio_stream::iter(vec![file_metadata]));

        request_stream.metadata_mut().append(
            "authorization",
            format!("Bearer {}", user_1_auth.access_token).parse()?,
        );

        let file = storage_client
            .create_file(request_stream)
            .await?
            .into_inner();

        // -- end setup

        // -- start execution

        let share_request = ShareFileRequest {
            user_ids: vec![user_2.id.to_string()],
            share_option: None,
            file_id: file.id,
        };

        let mut request = Request::new(share_request);

        request.metadata_mut().append(
            "authorization",
            format!("Bearer {}", user_1_auth.access_token).parse()?,
        );

        let response = storage_client.share_file(request).await;

        // -- end execution

        // -- validation

        assert!(response.is_ok());

        // -- end validation

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_list_shared_files(
        _: PoolOptions<Postgres>,
        conn_options: PgConnectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -- setup

        let db = Database::connect(conn_options.to_url_lossy()).await?;

        let temp_dir = tempdir::TempDir::new("sample")?;

        let mut user_1 = users::ActiveModel::new();

        user_1.email = Set("sample@email.com".into());
        user_1.password = Set("Randompassword1!".into());
        user_1.auth_type = Set(AuthType::BasicAuth);

        let _ = user_1.insert(&db).await?;

        let mut user_2 = users::ActiveModel::new();

        user_2.email = Set("sample2@email.com".into());
        user_2.password = Set("Randompassword1!".into());
        user_2.auth_type = Set(AuthType::BasicAuth);

        let user_2 = user_2.insert(&db).await?;

        let key = Hmac::new_from_slice(b"random-encryptio-key")?;

        let (_, storage_channel) = start_server(
            Server::builder().add_service(StorageService::new(&db, temp_dir.path(), key.clone())),
        )
        .await;

        let (_, auth_channel) =
            start_server(Server::builder().add_service(AuthService::new(&db, key.clone()))).await;

        let mut auth_client = AuthServiceClient::new(auth_channel);

        let user_1_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let user_2_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample2@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let mut storage_client = StorageServiceClient::new(storage_channel);

        for i in 0..3 {
            let file_content = br#"
            This is a sample .txt file
        "#;

            let file_metadata = CreateFileRequest {
                metadata: Some(lib_proto::storage::CreateFileMetadataRequest {
                    name: format!("test_file_{}.txt", i),
                    r#type: "text/plain".into(),
                    is_public: false,
                    size: file_content.len() as u32,
                }),
                chunk: Some(FileChunk {
                    chunk: file_content.to_vec(),
                }),
            };

            let mut request_stream = Request::new(tokio_stream::iter(vec![file_metadata]));

            request_stream.metadata_mut().append(
                "authorization",
                format!("Bearer {}", user_1_auth.access_token).parse()?,
            );

            let file = storage_client
                .create_file(request_stream)
                .await?
                .into_inner();

            let share_request = ShareFileRequest {
                user_ids: vec![user_2.id.to_string()],
                share_option: None,
                file_id: file.id,
            };

            let mut request = Request::new(share_request);

            request.metadata_mut().append(
                "authorization",
                format!("Bearer {}", user_1_auth.access_token).parse()?,
            );

            let _ = storage_client.share_file(request).await?;
        }

        // -- end setup

        // -- start execution

        let mut list_shared_files_request = Request::new(());

        list_shared_files_request.metadata_mut().append(
            "authorization",
            format!("Bearer {}", user_2_auth.access_token).parse()?,
        );

        let response = storage_client
            .list_shared_files(list_shared_files_request)
            .await?
            .into_inner()
            .try_collect::<Vec<_>>()
            .await?;

        // -- end execution

        // -- validation

        assert_eq!(response.len(), 3);

        // -- end validation

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_list_shared_public_files(
        _: PoolOptions<Postgres>,
        conn_options: PgConnectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -- setup

        let db = Database::connect(conn_options.to_url_lossy()).await?;

        let temp_dir = tempdir::TempDir::new("sample")?;

        let mut user_1 = users::ActiveModel::new();

        user_1.email = Set("sample@email.com".into());
        user_1.password = Set("Randompassword1!".into());
        user_1.auth_type = Set(AuthType::BasicAuth);

        let _ = user_1.insert(&db).await?;

        let mut user_2 = users::ActiveModel::new();

        user_2.email = Set("sample2@email.com".into());
        user_2.password = Set("Randompassword1!".into());
        user_2.auth_type = Set(AuthType::BasicAuth);

        let _ = user_2.insert(&db).await?;

        let key = Hmac::new_from_slice(b"random-encryptio-key")?;

        let (_, storage_channel) = start_server(
            Server::builder().add_service(StorageService::new(&db, temp_dir.path(), key.clone())),
        )
        .await;

        let (_, auth_channel) =
            start_server(Server::builder().add_service(AuthService::new(&db, key.clone()))).await;

        let mut auth_client = AuthServiceClient::new(auth_channel);

        let user_1_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let user_2_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample2@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let mut storage_client = StorageServiceClient::new(storage_channel);

        for i in 0..3 {
            let file_content = br#"
            This is a sample .txt file
        "#;

            let file_metadata = CreateFileRequest {
                metadata: Some(lib_proto::storage::CreateFileMetadataRequest {
                    name: format!("test_file_{}", i),
                    r#type: "text/plain".into(),
                    is_public: true,
                    size: file_content.len() as u32,
                }),
                chunk: Some(FileChunk {
                    chunk: file_content.to_vec(),
                }),
            };

            let mut request_stream = Request::new(tokio_stream::iter(vec![file_metadata]));

            request_stream.metadata_mut().append(
                "authorization",
                format!("Bearer {}", user_1_auth.access_token).parse()?,
            );

            let _ = storage_client
                .create_file(request_stream)
                .await?
                .into_inner();
        }
        // -- end setup

        // -- start execution

        let mut list_shared_files_request = Request::new(());

        list_shared_files_request.metadata_mut().append(
            "authorization",
            format!("Bearer {}", user_2_auth.access_token).parse()?,
        );

        let response = storage_client
            .list_shared_files(list_shared_files_request)
            .await?
            .into_inner()
            .try_collect::<Vec<_>>()
            .await?;

        // -- end execution

        // -- validation

        assert_eq!(response.len(), 3);

        // -- end validation

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_download_file(
        _: PoolOptions<Postgres>,
        conn_options: PgConnectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -- setup

        let db = Database::connect(conn_options.to_url_lossy()).await?;

        let temp_dir = tempdir::TempDir::new("sample")?;

        let mut user = users::ActiveModel::new();

        user.email = Set("sample@email.com".into());
        user.password = Set("Randompassword1!".into());
        user.auth_type = Set(AuthType::BasicAuth);

        let _ = user.insert(&db).await?;

        let key = Hmac::new_from_slice(b"random-encryptio-key")?;

        let (_, storage_channel) = start_server(
            Server::builder().add_service(StorageService::new(&db, temp_dir.path(), key.clone())),
        )
        .await;

        let (_, auth_channel) =
            start_server(Server::builder().add_service(AuthService::new(&db, key.clone()))).await;

        let mut auth_client = AuthServiceClient::new(auth_channel);

        let user_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let mut storage_client = StorageServiceClient::new(storage_channel);

        let file_content = br#"
            This is a sample .txt file
        "#;

        let file_metadata = CreateFileRequest {
            metadata: Some(lib_proto::storage::CreateFileMetadataRequest {
                name: "test_file.txt".into(),
                r#type: "text/plain".into(),
                is_public: true,
                size: file_content.len() as u32,
            }),
            chunk: Some(FileChunk {
                chunk: file_content.to_vec(),
            }),
        };

        let mut request_stream = Request::new(tokio_stream::iter(vec![file_metadata]));

        request_stream.metadata_mut().append(
            "authorization",
            format!("Bearer {}", user_auth.access_token).parse()?,
        );

        let file = storage_client
            .create_file(request_stream)
            .await?
            .into_inner();

        // -- end setup

        // -- start execution

        let mut download_file_request = Request::new(DownloadFileRequest { id: file.id });

        download_file_request.metadata_mut().append(
            "authorization",
            format!("Bearer {}", user_auth.access_token).parse()?,
        );

        let response = storage_client
            .download_file(download_file_request)
            .await?
            .into_inner()
            .try_collect::<Vec<_>>()
            .await?;

        let file_chunk = response.get(0).unwrap();

        // -- end execution

        // -- validation

        assert_eq!(file_chunk.chunk.len(), file_content.len());

        // -- end validation

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_delete_file(
        _: PoolOptions<Postgres>,
        conn_options: PgConnectOptions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -- setup

        let db = Database::connect(conn_options.to_url_lossy()).await?;

        let temp_dir = tempdir::TempDir::new("sample")?;

        let mut user = users::ActiveModel::new();

        user.email = Set("sample@email.com".into());
        user.password = Set("Randompassword1!".into());
        user.auth_type = Set(AuthType::BasicAuth);

        let _ = user.insert(&db).await?;

        let key = Hmac::new_from_slice(b"random-encryptio-key")?;

        let (_, storage_channel) = start_server(
            Server::builder().add_service(StorageService::new(&db, temp_dir.path(), key.clone())),
        )
        .await;

        let (_, auth_channel) =
            start_server(Server::builder().add_service(AuthService::new(&db, key.clone()))).await;

        let mut auth_client = AuthServiceClient::new(auth_channel);

        let user_auth = auth_client
            .basic_login(AuthBasicLoginRequest {
                email: "sample@email.com".into(),
                password: "Randompassword1!".into(),
            })
            .await?
            .into_inner();

        let mut storage_client = StorageServiceClient::new(storage_channel);

        let file_content = br#"
            This is a sample .txt file
        "#;

        let file_metadata = CreateFileRequest {
            metadata: Some(lib_proto::storage::CreateFileMetadataRequest {
                name: "test_file.txt".into(),
                r#type: "text/plain".into(),
                is_public: true,
                size: file_content.len() as u32,
            }),
            chunk: Some(FileChunk {
                chunk: file_content.to_vec(),
            }),
        };

        let mut request_stream = Request::new(tokio_stream::iter(vec![file_metadata]));

        request_stream.metadata_mut().append(
            "authorization",
            format!("Bearer {}", user_auth.access_token).parse()?,
        );

        let file = storage_client
            .create_file(request_stream)
            .await?
            .into_inner();

        // -- end setup

        // -- start execution

        let mut delete_file_request = Request::new(DeleteFileRequest { id: file.id });

        delete_file_request.metadata_mut().append(
            "authorization",
            format!("Bearer {}", user_auth.access_token).parse()?,
        );

        let response = storage_client.delete_file(delete_file_request).await;

        // -- end execution

        // -- validation

        assert!(response.is_ok());

        let db_files = file::Entity::find().all(&db).await?;

        assert_eq!(db_files.len(), 0);

        let directory = std::fs::read_dir(temp_dir.path())?.collect::<Vec<_>>();

        assert_eq!(directory.len(), 0);

        // -- end validation

        Ok(())
    }
}
