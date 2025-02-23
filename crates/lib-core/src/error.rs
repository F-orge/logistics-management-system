use derive_more::From;
use tonic::Status;

#[derive(From, Debug)]
pub enum Error {
    Custom(Box<dyn std::error::Error>),
    Database(sqlx::Error),
    Query(sea_query::error::Error),
    Io(std::io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<Error> for Status {
    fn from(err: Error) -> Self {
        match err {
            Error::Database(err) => {
                tracing::error!("{}", err);
                match err {
                    sqlx::Error::RowNotFound => Status::not_found("Row not found"),
                    sqlx::Error::ColumnNotFound(col) => {
                        Status::invalid_argument(format!("Column not found {}", col))
                    }
                    _ => Status::internal("Database related error"),
                }
            }
            Error::Query(err) => {
                tracing::warn!("{}", err);
                match err {
                    sea_query::error::Error::ColValNumMismatch { col_len, val_len } => {
                        Status::invalid_argument(format!(
                            "Column mismatch: {} {}",
                            col_len, val_len
                        ))
                    }
                }
            }
            Error::Io(err) => {
                tracing::error!("{}", err);
                return Status::internal("Internal IO Error");
            }
            Error::Custom(err) => {
                tracing::error!("{}", err);
                Status::internal("Internal server error")
            }
        }
    }
}
