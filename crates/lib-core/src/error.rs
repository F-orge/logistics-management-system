use derive_more::From;
use sqlx::types::uuid;
use tonic::Status;

#[derive(From, Debug)]
pub enum Error {
    // -- Unhandled Error
    Custom(Box<dyn std::error::Error + Send + Sync>),
    // -- Database Error
    SeaOrm(sea_orm::DbErr),
    RowNotFound,
    // -- File Io
    Io(std::io::Error),
    // -- Tonic Error
    Tonic(tonic::Status),
    // -- Validation Error,
    Uuid(uuid::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<Error> for Status {
    fn from(err: Error) -> Self {
        match err {
            Error::SeaOrm(err) => {
                tracing::info!("{}", err);
                match err {
                    sea_orm::DbErr::RecordNotFound(id) => {
                        Status::not_found(format!("Row not found {}", id))
                    }
                    _ => Status::internal("Internal Database Error"),
                }
            }
            Error::RowNotFound => Status::not_found("Row not found"),
            Error::Io(err) => {
                tracing::error!("{}", err);
                return Status::internal("Internal IO Error");
            }
            Error::Tonic(err) => {
                tracing::error!("{}", err);
                return err;
            }
            Error::Uuid(err) => {
                tracing::warn!("{}", err);
                return Status::invalid_argument("Cannot parse Uuid");
            }
            Error::Custom(err) => {
                tracing::error!("{}", err);
                Status::internal("Internal server error")
            }
        }
    }
}
