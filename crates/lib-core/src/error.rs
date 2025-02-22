use derive_more::From;
use tonic::Status;

#[derive(From, Debug)]
pub enum Error {
    Custom(Box<dyn std::error::Error>),
    Database(sqlx::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<Error> for Status {
    fn from(err: Error) -> Self {
        match err {
            Error::Database(err) => {
                tracing::error!("{}", err);
                Status::internal("Internal server error")
            }
            Error::Custom(err) => {
                tracing::error!("{}", err);
                Status::internal("Internal server error")
            }
        }
    }
}
