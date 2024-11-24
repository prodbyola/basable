use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum AppError {
    InitError(String),
    PersistentStorageError(String),
    HttpError(StatusCode, String),
    ServerError(String),
}

impl AppError {
    pub fn not_implemented() -> Self {
        Self::HttpError(
            StatusCode::NOT_IMPLEMENTED,
            String::from("feature not implemented"),
        )
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InitError(msg) => write!(f, "{msg}"),
            AppError::PersistentStorageError(msg) => write!(f, "{msg}"),
            AppError::ServerError(msg) => write!(f, "{msg}"),
            AppError::HttpError(code, msg) => write!(f, "{code}: {msg}"),
        }
    }
}

impl From<mysql::Error> for AppError {
    fn from(value: mysql::Error) -> Self {
        AppError::ServerError(value.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            AppError::HttpError(code, msg) => (code, msg),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        resp.into_response()
    }
}
