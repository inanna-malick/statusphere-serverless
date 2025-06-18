use anyhow::anyhow;
use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Backend error: {0:?}")]
    Misc(#[from] anyhow::Error),
    #[error(transparent)]
    Worker(#[from] worker::Error),
    // #[error(transparent)]
    // WorkerKV(#[from] worker::kv::KvError),
    #[error(transparent)]
    SessionManagement(#[from] tower_sessions::session::Error),
    #[error("Something went wrong in the Oauth flow: {0}")]
    Oauth(#[from] atrium_oauth::Error),
    #[error("authorization required")]
    NoSessionAuth,
    #[error("admin endpoint - authorization required")]
    NoAdminAuth,
    #[error("authentication error, maybe your session is invalid")]
    AuthenticationInvalid,
}

impl<T: std::fmt::Debug> From<atrium_xrpc::Error<T>> for AppError {
    fn from(value: atrium_xrpc::Error<T>) -> Self {
        match value {
            atrium_xrpc::Error::Authentication(_header_value) => AppError::AuthenticationInvalid,
            e => anyhow!("atrium xrpc layer error: {:?}", e).into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            match &self {
                AppError::NoAdminAuth | AppError::NoSessionAuth => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            format!("Error: {self}"),
        )
            .into_response()
    }
}
