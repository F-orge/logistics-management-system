use askama::Template;
use askama_axum::IntoResponse;
use axum::{http::StatusCode, response::Html};
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
    // -- Validation
    Garde(garde::Report),
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
                println!("{}", err);
                match err {
                    sea_orm::DbErr::RecordNotFound(id) => {
                        Status::not_found(format!("Row not found {}", id))
                    }
                    _ => Status::internal("Internal Database Error"),
                }
            }
            Error::Garde(err) => panic!("err"),
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

// askama error

pub type AskamaResult<T> = std::result::Result<T, AskamaError>;

#[derive(Debug, Template)]
#[template(path = "full-page-error.html.jinja")]
pub struct FullError {
    pub code: u16,
    pub message: String,
}

#[derive(Debug, Template)]
#[template(path = "alert-error.html.jinja")]
pub struct AlertNotification {
    pub alert_type: AlertNotificationType,
    pub code: u16,
    pub title: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AlertNotificationType {
    Info,
    Error,
}

pub enum AskamaError {
    FullError(FullError),
    AlertNotification(AlertNotification),
}

impl From<Error> for AskamaError {
    fn from(value: Error) -> Self {
        match value {
            Error::SeaOrm(err) => {
                println!("{}", err);
                match err {
                    sea_orm::DbErr::RecordNotFound(id) => {
                        Self::AlertNotification(AlertNotification {
                            alert_type: AlertNotificationType::Error,
                            code: StatusCode::NOT_FOUND.as_u16(),
                            title: "Not found".into(),
                            message: format!("Row not found id: {}", id),
                        })
                    }
                    _ => Self::FullError(FullError {
                        code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Internal server error".into(),
                    }),
                }
            }
            Error::Garde(err) => Self::AlertNotification(AlertNotification {
                alert_type: AlertNotificationType::Error,
                code: StatusCode::BAD_REQUEST.as_u16(),
                title: "Validation Error".into(),
                message: err.to_string(),
            }),
            Error::RowNotFound => Self::AlertNotification(AlertNotification {
                alert_type: AlertNotificationType::Error,
                code: StatusCode::NOT_FOUND.as_u16(),
                title: "Not found".into(),
                message: "Row not found id".into(),
            }),
            Error::Io(err) => {
                tracing::error!("{}", err);
                Self::FullError(FullError {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "Internal server error".into(),
                })
            }
            Error::Uuid(err) => {
                tracing::warn!("{}", err);
                Self::AlertNotification(AlertNotification {
                    alert_type: AlertNotificationType::Error,
                    code: StatusCode::BAD_REQUEST.as_u16(),
                    title: "UUID Error".into(),
                    message: "Cannot parse UUID".into(),
                })
            }
            Error::Tonic(err) => {
                panic!("Not implemented")
            }
            Error::Custom(err) => {
                tracing::error!("{}", err);
                Self::FullError(FullError {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "Internal server error".into(),
                })
            }
        }
    }
}

impl IntoResponse for AskamaError {
    fn into_response(self) -> askama_axum::Response {
        match self {
            Self::FullError(err) => Html(err.render().unwrap_or_default()).into_response(),
            Self::AlertNotification(err) => Html(err.render().unwrap_or_default()).into_response(),
        }
    }
}
