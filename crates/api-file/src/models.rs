use axum_typed_multipart::{FieldData, TryFromMultipart};
use sea_orm::schema;
use serde::Deserialize;
use tempfile::NamedTempFile;
use utoipa::{IntoParams, ToSchema};

#[derive(TryFromMultipart)]
pub struct UploadFile {
    pub file: FieldData<NamedTempFile>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadFileRequest {
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    pub file: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct PaginateFiles {
    #[param(default = 5, example = 5)]
    pub limit: u64,
    #[param(default = 0, example = 0)]
    pub page: u64,
    #[param(default = false, example = false)]
    pub shared: bool,
    #[param(default = true, example = true)]
    pub public: bool,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchFiles {
    #[param(default = 5, example = 5)]
    pub limit: u64,
    #[param(default = 0, example = 0)]
    pub page: u64,
    #[param(default = "", example = "")]
    pub query: String,
}
