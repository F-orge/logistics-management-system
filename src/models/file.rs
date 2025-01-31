use sea_orm::Set;
use tonic::Status;

use super::{
    _entities::file::{ActiveModel, Model},
    _proto::file_management::{
        CreateFileRequest, FileError, FileErrorCode, FileResponse, FileStatus as GrpcFileStatus,
        UpdateFileRequest, UpdateFileStatusRequest,
    },
};

// Request -> ActiveModel conversions
impl TryInto<ActiveModel> for CreateFileRequest {
    type Error = Status;

    fn try_into(self) -> Result<ActiveModel, Self::Error> {
        let status = match GrpcFileStatus::try_from(self.status) {
            Ok(GrpcFileStatus::Active) => "ACTIVE",
            Ok(GrpcFileStatus::Archived) => "ARCHIVED",
            Ok(GrpcFileStatus::Deleted) => "DELETED",
            Ok(GrpcFileStatus::Unspecified) => {
                return Err(Status::invalid_argument("Unspecified file status"))
            }
            Err(_) => return Err(Status::invalid_argument("Invalid file status")),
        };

        Ok(ActiveModel {
            name: Set(self.name),
            path: Set(self.path),
            mime_type: Set(self.mime_type),
            size: Set(self.size as i64),
            status: Set(status.to_string()),
            system_origin: Set(self.system_origin),
            created_by: Set(self.created_by),
            tags: Set(self.tags.join(",")),
            description: Set(Some(self.description)),
            last_accessed_at: Set(None),
            deleted_at: Set(None),
            ..Default::default()
        })
    }
}

impl TryInto<ActiveModel> for UpdateFileRequest {
    type Error = Status;

    fn try_into(self) -> Result<ActiveModel, Self::Error> {
        let mut active_model = ActiveModel {
            id: Set(self
                .file_id
                .parse()
                .map_err(|_| Status::invalid_argument("Invalid UUID format"))?),
            ..Default::default()
        };

        if let Some(name) = self.name {
            active_model.name = Set(name);
        }
        if let Some(path) = self.path {
            active_model.path = Set(path);
        }
        if let Some(description) = self.description {
            active_model.description = Set(Some(description));
        }
        if !self.tags.is_empty() {
            active_model.tags = Set(self.tags.join(","));
        }

        Ok(active_model)
    }
}

impl TryInto<ActiveModel> for UpdateFileStatusRequest {
    type Error = Status;

    fn try_into(self) -> Result<ActiveModel, Self::Error> {
        let status = match GrpcFileStatus::try_from(self.status) {
            Ok(GrpcFileStatus::Active) => "ACTIVE",
            Ok(GrpcFileStatus::Archived) => "ARCHIVED",
            Ok(GrpcFileStatus::Deleted) => "DELETED",
            Ok(GrpcFileStatus::Unspecified) => {
                return Err(Status::invalid_argument("Unspecified file status"))
            }
            Err(_) => return Err(Status::invalid_argument("Invalid file status")),
        };

        Ok(ActiveModel {
            id: Set(self
                .file_id
                .parse()
                .map_err(|_| Status::invalid_argument("Invalid UUID format"))?),
            status: Set(status.to_string()),
            ..Default::default()
        })
    }
}

// Model -> Response conversions
impl TryInto<FileResponse> for Model {
    type Error = Status;

    fn try_into(self) -> Result<FileResponse, Self::Error> {
        let status = match self.status.as_str() {
            "ACTIVE" => GrpcFileStatus::Active as i32,
            "ARCHIVED" => GrpcFileStatus::Archived as i32,
            "DELETED" => GrpcFileStatus::Deleted as i32,
            _ => return Err(Status::internal("Invalid file status in database")),
        };

        Ok(FileResponse {
            id: self.id.to_string(),
            name: self.name,
            path: self.path,
            mime_type: self.mime_type,
            size: self.size as u64,
            status,
            system_origin: self.system_origin,
            created_by: self.created_by,
            created_at: self.created_at.to_string(),
            updated_at: self.updated_at.to_string(),
            tags: self.tags.split(',').map(String::from).collect(),
            description: self.description.unwrap_or_default(),
            last_accessed_at: self.last_accessed_at.map(|t| t.to_string()),
            deleted_at: self.deleted_at.map(|t| t.to_string()),
        })
    }
}

// Helper function for error conversion
pub fn to_file_error(status: Status, file_id: String) -> FileError {
    let error_code = match status.code() {
        tonic::Code::NotFound => FileErrorCode::FileErrorNotFound,
        tonic::Code::PermissionDenied => FileErrorCode::FileErrorPermissionDenied,
        tonic::Code::InvalidArgument => FileErrorCode::FileErrorInvalidStatus,
        tonic::Code::Internal => FileErrorCode::FileErrorStorageFailed,
        _ => FileErrorCode::FileErrorUnspecified,
    };

    FileError {
        file_id,
        error_message: status.message().to_string(),
        error_code: error_code as i32,
    }
}
