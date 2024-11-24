use common::error::AppError;
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::get_env;

use crate::http::app::app;

mod foundation;
mod http;
mod state;
mod user;
mod utils;

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

    let port = get_env("BASABLE_PORT").map_err(|err| AppError::InitError(err.to_string()))?;
    let app = app()?;

    if let Ok(listener) = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await {
        if let Ok(addr) = listener.local_addr() {
            tracing::debug!("listening on {addr}");
        }

        axum::serve(listener, app)
            .await
            .map_err(|err| AppError::InitError(err.to_string()))?;

        if !cfg!(debug_assertions) {
            let dm =
                get_env("DEPLOYMENT_MODE").map_err(|err| AppError::InitError(err.to_string()))?;
            let dm = DeploymentMode::from(dm);

            if dm.is_local() {
                let url = format!("http://localhost:{port}");
                match webbrowser::open(&url) {
                    Ok(_) => tracing::debug!("Browser launched successfully"),
                    Err(err) => tracing::debug!("Unable to lauch browser: {err}"),
                }
            }
        }
    }

    Ok(())
}
