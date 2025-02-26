use std::path::{Path, PathBuf};

use futures::{StreamExt, TryStreamExt};
use lib_core::error::Error;
use lib_entity::{file, file_access, prelude::*};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, Set, TransactionTrait,
};
use sqlx::{types::Uuid, PgPool, Pool, Postgres};
use tokio::{fs, sync::mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use lib_proto::storage::{
    file_metadata_request,
    storage_service_server::{StorageService as GRPCStorageService, StorageServiceServer},
    CreateFileRequest, DeleteFileRequest, DownloadFileRequest, FileChunk, FileMetadata,
    FileMetadataRequest,
};

pub struct StorageService {
    db: DatabaseConnection,
    directory: PathBuf,
}

impl StorageService {
    pub fn new(db: &DatabaseConnection, directory: &Path) -> StorageServiceServer<Self> {
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
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

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
                file.owner_id = todo!("implement on how to get the user_id by token");
                file.r#type = Set(create_metadata.r#type);
                file.size = Set(create_metadata.size as i32);

                let file = file.insert(&trx).await.map_err(Error::SeaOrm)?;

                let mut file_access = file_access::ActiveModel::new();

                file_access.file_id = Set(file.id);
                file_access.user_id = Set(todo!("implement on how to get the user_id by token"));

                let _ = file_access.insert(&trx).await.map_err(Error::SeaOrm)?;

                has_inserted_to_db = true;

                metadata = FileMetadata {
                    id: file.id.to_string(),
                    name: file.name,
                    r#type: file.r#type,
                    size: file.size as u32,
                    is_public: file.is_public,
                    owner_id: file.owner_id.to_string(),
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
        _request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<Self::ListOwnedFilesStream>, tonic::Status> {
        let mut user_files = File::find()
            .filter(file::Column::OwnerId.eq("Todo: owner_id"))
            .stream(&self.db)
            .await
            .map_err(Error::SeaOrm)?;

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<FileMetadata, Status>>(1024 * 64);

        while let Some(row) = user_files.try_next().await.map_err(Error::SeaOrm)? {
            let item: file::Model = row.into();
            tx.send(Ok(FileMetadata {
                id: item.id.to_string(),
                name: item.name,
                r#type: item.r#type,
                size: item.size as u32,
                is_public: item.is_public,
                owner_id: item.owner_id.to_string(),
            }))
            .await
            .map_err(|err| Error::Custom(Box::new(err)))?;
        }
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn list_shared_files(
        &self,
        request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<Self::ListSharedFilesStream>, tonic::Status> {
        let mut shared_files = File::find()
            .inner_join(FileAccess)
            .filter(file::Column::OwnerId.ne("TODO: owner_id"))
            .filter(file_access::Column::UserId.eq("TODO: owner_id"))
            .stream(&self.db)
            .await
            .map_err(Error::SeaOrm)?;

        let (tx, rx) = tokio::sync::mpsc::channel::<Result<FileMetadata, Status>>(1024 * 64);

        while let Some(row) = shared_files.try_next().await.map_err(Error::SeaOrm)? {
            let item: file::Model = row.into();
            tx.send(Ok(FileMetadata {
                id: item.id.to_string(),
                name: item.name,
                r#type: item.r#type,
                size: item.size as u32,
                is_public: item.is_public,
                owner_id: item.owner_id.to_string(),
            }))
            .await
            .map_err(|err| Error::Custom(Box::new(err)))?;
        }
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn share_file(
        &self,
        request: tonic::Request<lib_proto::storage::ShareFileRequest>,
    ) -> std::result::Result<Response<()>, tonic::Status> {
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let payload = request.into_inner();

        // check if we have the right properties.
        if payload.user_ids.len() == 0 && payload.share_option.is_none() {
            return Err(Status::invalid_argument(
                "user_ids and share options are both empty",
            ));
        }

        if let Some(share_option) = payload.share_option {
            match share_option {
                lib_proto::share_file_request::ShareOption::IsPublic(is_public) => {
                    let mut file = File::find()
                        .filter(file::Column::Id.eq(payload.file_id))
                        .one(&trx)
                        .await
                        .map_err(Error::SeaOrm)?
                        .ok_or(Error::RowNotFound)?
                        .into_active_model();

                    file.is_public = Set(is_public);

                    let _ = file.update(&trx).await.map_err(Error::SeaOrm)?;

                    trx.commit().await.map_err(Error::SeaOrm)?;

                    return Ok(Response::new(()));
                }
            }
        }

        for user_id in payload.user_ids {
            let mut insert_stmt = file_access::ActiveModel::new();

            insert_stmt.file_id = Set(payload.file_id.parse().unwrap());
            insert_stmt.user_id = Set(user_id.parse().unwrap());

            let _ = insert_stmt.insert(&trx).await.map_err(Error::SeaOrm)?;
        }

        trx.commit().await.map_err(Error::SeaOrm)?;

        Ok(Response::new(()))
    }

    async fn download_file(
        &self,
        request: tonic::Request<DownloadFileRequest>,
    ) -> std::result::Result<tonic::Response<Self::DownloadFileStream>, tonic::Status> {
        let file_id = request.into_inner().id.parse::<Uuid>().unwrap();

        let file = File::find()
            .filter(file::Column::Id.eq(file_id))
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
        let payload = request.into_inner();

        let metadata = match payload.request {
            Some(file_metadata_request::Request::Id(id)) => {
                let file_id = id
                    .parse::<Uuid>()
                    .map_err(|err| lib_core::error::Error::Custom(Box::new(err)))?;

                let file = File::find()
                    .filter(file::Column::Id.eq(id))
                    .one(&self.db)
                    .await
                    .map_err(Error::SeaOrm)?
                    .ok_or(Error::RowNotFound)?;

                FileMetadata {
                    id: file.id.to_string(),
                    name: file.name,
                    r#type: file.r#type,
                    size: file.size as u32,
                    is_public: file.is_public,
                    owner_id: file.owner_id.to_string(),
                }
            }
            Some(file_metadata_request::Request::Name(name)) => {
                let file = File::find()
                    .filter(file::Column::Name.eq(name))
                    .one(&self.db)
                    .await
                    .map_err(Error::SeaOrm)?
                    .ok_or(Error::RowNotFound)?;

                FileMetadata {
                    id: file.id.to_string(),
                    name: file.name,
                    r#type: file.r#type,
                    size: file.size as u32,
                    is_public: file.is_public,
                    owner_id: file.owner_id.to_string(),
                }
            }
            None => return Err(Status::invalid_argument("Missing request parameters")),
        };

        Ok(Response::new(metadata))
    }

    async fn delete_file(
        &self,
        request: tonic::Request<DeleteFileRequest>,
    ) -> Result<Response<()>, Status> {
        let trx = self.db.begin().await.map_err(Error::SeaOrm)?;

        let payload = request.into_inner();

        let file_id = payload
            .id
            .parse::<Uuid>()
            .map_err(|err| lib_core::error::Error::Custom(Box::new(err)))?;

        let file = File::find()
            .filter(file::Column::Id.eq(file_id))
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
/*
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

    use lib_proto::{
        auth::{self, auth_service_client::AuthServiceClient, AuthBasicLoginRequest},
        storage::{storage_service_client::StorageServiceClient, CreateFileRequest},
        AuthResponse, ShareFileRequest,
    };

    use service_authentication::AuthService;

    use lib_utils::test::start_server;

    use super::*;

    async fn create_dummy_user(
        db: &Pool<Postgres>,
        email: &str,
        password: &str,
    ) -> lib_core::error::Result<()> {
        let mut conn = lib_core::database::aquire_connection(&db).await?;

        let mut trx = lib_core::database::start_transaction(&mut conn).await?;

        lib_utils::db::setup_db(&mut trx, &MetadataMap::new())
            .await
            .map_err(lib_core::error::Error::Database)?;

        let (query, values) = Query::insert()
            .into_table((Alias::new("auth"), Alias::new("basic_user")))
            .columns([Alias::new("email"), Alias::new("password")])
            .values([email.into(), password.into()])
            .map_err(lib_core::error::Error::Query)?
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&query, values)
            .execute(&mut *trx)
            .await
            .map_err(lib_core::error::Error::Database)?;

        lib_core::database::commit_transaction(trx).await?;

        Ok(())
    }

    fn setup_env_variables() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        std::env::set_var("RUST_POSTGRES_JWT_SECRET", "randomsecret");
        std::env::set_var("RUST_POSTGRES_JWT_AUDIENCE", "management.com");
        std::env::set_var("RUST_POSTGRES_JWT_ISSUER", "web");
        std::env::set_var("RUST_POSTGRES_JWT_EXPIRY", "3600");
        Ok(())
    }

    async fn setup_client_with_token(
        db: &Pool<Postgres>,
        email: &str,
        password: &str,
    ) -> lib_core::error::Result<(
        TempDir,
        StorageServiceClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    )> {
        let (_, channel) = start_server(Server::builder().add_service(AuthService::new(db))).await;
        let mut client = AuthServiceClient::new(channel);

        // register the user first
        create_dummy_user(db, email, password).await?;

        let auth_response = client
            .basic_login(AuthBasicLoginRequest {
                email: email.to_string(),
                password: password.to_string(),
            })
            .await
            .map_err(|err| lib_core::error::Error::Tonic(err))?
            .into_inner();

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

        // get the token
        Ok((tmp_dir, client))
    }

    async fn create_dummy_file(
        client: &mut StorageServiceClient<
            InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>,
        >,
    ) -> lib_core::error::Result<FileMetadata> {
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

        Ok(client
            .create_file(request_stream)
            .await
            .map_err(lib_core::error::Error::Tonic)?
            .into_inner())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_storage_create_file(db: Pool<Postgres>) -> lib_core::error::Result<()> {
        // -- setup
        setup_env_variables()?;

        let (tmp_dir, mut client) =
            setup_client_with_token(&db, "sample@email.com", "RandomPassword1!").await?;

        // send one chunk to the backend
        let file_content = b"HELLO MY NAME IS JOHN DOE. i am a file!!! :3";

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

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_storage_download_file(db: Pool<Postgres>) -> lib_core::error::Result<()> {
        // -- setup
        setup_env_variables()?;

        let (_tmp_dir, mut client) =
            setup_client_with_token(&db, "sample@email.com", "RandomPassword1!").await?;

        // create a file
        let metadata = create_dummy_file(&mut client).await?;

        let metadata_request = Request::new(DownloadFileRequest { id: metadata.id });

        // download file
        let response = client.download_file(metadata_request).await.unwrap();

        let chunks: Vec<FileChunk> = response.into_inner().try_collect().await.unwrap();

        let content = chunks
            .into_iter()
            .flat_map(|chunk| chunk.chunk)
            .collect::<Vec<u8>>();

        assert_eq!(
            content,
            br#"
            This is a sample .txt file
        "#
        );

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_list_owned_files(db: Pool<Postgres>) -> lib_core::error::Result<()> {
        setup_env_variables()?;

        let (tmp_dir, mut client) =
            setup_client_with_token(&db, "sample@email.com", "RandomPassword1!").await?;

        let file_1 = create_dummy_file(&mut client).await?;
        let file_2 = create_dummy_file(&mut client).await?;
        let file_3 = create_dummy_file(&mut client).await?;

        let mut stream = client
            .list_owned_files(Request::new(()))
            .await
            .unwrap()
            .into_inner();

        let files = stream.try_collect::<Vec<_>>().await.unwrap();

        // must have 3 files
        assert_eq!(files.len(), 3);

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_share_file_to_public(db: Pool<Postgres>) -> lib_core::error::Result<()> {
        setup_env_variables()?;

        let (_temp_dir, mut john_client) =
            setup_client_with_token(&db, "john@email.com", "RandomPassword1!").await?;

        let file = create_dummy_file(&mut john_client).await?;

        assert_eq!(file.is_public, false);

        // convert file to public

        let request = ShareFileRequest {
            file_id: file.id.clone(),
            user_ids: vec![],
            share_option: Some(lib_proto::share_file_request::ShareOption::IsPublic(true)),
        };

        let response = john_client.share_file(request).await;

        assert_eq!(response.is_ok(), true);

        // retrieve the file to see if public

        let public_file = john_client
            .get_file_metadata(FileMetadataRequest {
                request: Some(file_metadata_request::Request::Id(file.id)),
            })
            .await
            .unwrap()
            .into_inner();

        assert_eq!(public_file.is_public, true);

        Ok(())
    }

    /*
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
    */
}
*/
