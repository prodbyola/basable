use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};
use base::user::User;
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::get_env;

use crate::http::app::app;

mod base;
mod globals;
mod http;
mod imp;
mod tests;
mod utils;

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

enum DeploymentMode {
    Cloud,
    Local,
}
impl DeploymentMode {
    fn is_local(&self) -> bool {
        match self {
            DeploymentMode::Local => true,
            _ => false,
        }
    }
}
impl From<String> for DeploymentMode {
    fn from(value: String) -> Self {
        if value == "cloud" {
            return Self::Cloud;
        }

        Self::Local
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "basable=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = get_env("BASABLE_PORT");
    let app = app()?;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::InitError(err.to_string()))?;

    if !cfg!(debug_assertions) {
        let dm = get_env("DEPLOYMENT_MODE");
        let dm = DeploymentMode::from(dm);

        if dm.is_local() {
            let url = format!("http://localhost:{port}");
            match webbrowser::open(&url) {
                Ok(_) => println!("Browser launched successfully"),
                Err(err) => println!("Error launching browser: {err}"),
            }
        }
    }

    Ok(())
}
