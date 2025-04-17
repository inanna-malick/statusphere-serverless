use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Misc(#[from] anyhow::Error),
    #[error(transparent)]
    Worker(#[from] worker::Error),
    #[error(transparent)]
    SessionManagement(#[from] tower_sessions::session::Error),
    #[error("Something went wrong in the Oauth flow: {0}")]
    Oauth(#[from] atrium_oauth::Error),
    #[error("Missing required websocket upgrade header")]
    BadWebSocketUpgrade,
    #[error("authorization required")]
    NoSessionAuth,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            match &self {
                AppError::BadWebSocketUpgrade => StatusCode::UPGRADE_REQUIRED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            format!("Error: {self}"),
        )
            .into_response()
    }
}
