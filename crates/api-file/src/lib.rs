use axum::{
    Json,
    extract::{Query, State},
};
use axum_typed_multipart::TypedMultipart;
use lib_core::{
    AppState,
    error::{Error, ErrorResponse},
    result::{PaginatedResult, Result},
};
use lib_entity::generated::file;
use lib_security::JWTClaim;
use models::{PaginateFiles, SearchFiles, UploadFile};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait,
    QueryFilter, Set, TransactionTrait, sea_query::expr,
};
use sha2::{Digest, Sha256};
use std::io::Read;
use utoipa_axum::{router::OpenApiRouter, routes};
pub mod models;

#[utoipa::path(
    post,
    security(
        ("bearer" = ["file:upload"])
    ),
    operation_id = "UploadFile",
    request_body(content = models::UploadFileRequest, content_type = "multipart/form-data"),
    tag = "File Management",
    path = "/upload"
)]
async fn upload(
    jwt: JWTClaim,
    State(AppState {
        db, storage_path, ..
    }): State<AppState>,
    TypedMultipart(UploadFile { file }): TypedMultipart<UploadFile>,
) -> Result<String> {
    let metadata = file.metadata.clone();

    let mut file = file.contents.into_file();

    let mut contents = vec![];

    _ = file.read_to_end(&mut contents).map_err(Error::Io)?;

    let mut hasher = Sha256::new();

    hasher.update(&contents);

    let file_hash = format!("{:x}", hasher.finalize());

    let path = storage_path.join(file_hash);

    let trx = db.begin().await.map_err(Error::SeaOrm)?;

    let mut active_model = file::ActiveModel::new();

    active_model.name =
        Set(metadata
            .file_name
            .ok_or(Error::SeaOrm(sea_orm::DbErr::AttrNotSet(
                "file name not found".into(),
            )))?);
    active_model.file_path = Set(path
        .to_str()
        .ok_or(Error::SeaOrm(sea_orm::DbErr::AttrNotSet(
            "file path not found".into(),
        )))?
        .to_string());
    active_model.owner_id = Set(jwt.subject);
    active_model.shared_to = Set(vec![]);
    active_model.r#type =
        Set(metadata
            .content_type
            .ok_or(Error::SeaOrm(sea_orm::DbErr::AttrNotSet(
                "content type not found".into(),
            )))?);

    _ = active_model.insert(&trx).await.map_err(Error::SeaOrm)?;

    _ = trx.commit().await.map_err(Error::SeaOrm)?;

    if tokio::fs::try_exists(&path).await.map_err(Error::Io)? {
        return Ok("already exists. skipping".into());
    }

    //  write file to disk
    _ = tokio::fs::write(path, contents).await.map_err(Error::Io)?;

    Ok("sample".into())
}

#[utoipa::path(
    get,
    operation_id = "DownloadFile",
    tag = "File Management",
    path = "/download",
    responses(
        (status = 200, content_type = "application/octet-stream"),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn download() {}

#[utoipa::path(
    get, 
    tag = "File Management", 
    security(
        ("bearer" = ["file:read","file:bypass"])
    ),
    params(PaginateFiles),
    path = "/",
    responses(
        (status = 200, body = PaginatedResult<Vec<file::Model>>),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn read(
    jwt: JWTClaim,
    Query(PaginateFiles {
        limit,
        page,
        shared,
        public,
    }): Query<PaginateFiles>,
    State(AppState { db, .. }): State<AppState>,
) -> Result<Json<PaginatedResult<Vec<file::Model>>>> {
    // get the files that are owned by the user, is public and shared to the subject.
    // note: if file_permission has bypass flag, return all

    let mut conditions = Condition::any();

    conditions = conditions.add(file::Column::OwnerId.eq(jwt.subject));

    if public {
        conditions = conditions.add(file::Column::IsPublic.eq(true));
    }

    if shared {
        conditions = conditions.add(expr::Expr::cust(format!(
            r#"'{}'::uuid = ANY({})"#,
            jwt.subject, "shared_to"
        )));
    }

    let files = file::Entity::find()
        .filter(conditions)
        .paginate(&db, limit)
        .fetch_page(page)
        .await
        .map_err(Error::SeaOrm)?;

    Ok(Json(PaginatedResult {
        total: files.len() as u32,
        items: files,
    }))
}

#[utoipa::path(
    get, 
    security(
        ("bearer" = ["file:read","file:bypass"])
    ),
    tag = "File Management", 
    path = "/search",
    params(SearchFiles),
    responses(
        (status = 200, body = PaginatedResult<Vec<file::Model>>),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn search(
    jwt: JWTClaim,
    Query(SearchFiles { limit, page, query }): Query<SearchFiles>,
    State(AppState { db, .. }): State<AppState>,
) -> Result<Json<PaginatedResult<Vec<file::Model>>>> {
    let files = file::Entity::find()
        .filter(file::Column::Name.like(format!("%{}%", query)))
        .filter(
            Condition::any()
                .add(file::Column::OwnerId.eq(jwt.subject))
                .add(expr::Expr::cust(format!(
                    r#"'{}'::uuid = ANY({})"#,
                    jwt.subject, "shared_to"
                ))),
        )
        .paginate(&db, limit)
        .fetch_page(page)
        .await
        .map_err(Error::SeaOrm)?;

    Ok(Json(PaginatedResult {
        total: files.len() as u32,
        items: files,
    }))
}

#[utoipa::path(
    get, 
    tag = "File Management", 
    path = "/{id}",
    responses(
        (status = 200, body = file::Model),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn one() {}

#[utoipa::path(
    patch, 
    tag = "File Management", 
    path = "/{id}",
    responses(
        (status = 200, body = file::Model),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn update() {}

#[utoipa::path(
    delete, 
    tag = "File Management", 
    path = "/{id}",
    responses(
        (status = 204),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn remove() {}

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(upload))
        .routes(routes!(download))
        .routes(routes!(read))
        .routes(routes!(search))
        .routes(routes!(one))
        .routes(routes!(update))
        .routes(routes!(remove))
}
