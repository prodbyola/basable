use base::user::User;
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::http::app::app;

mod base;
mod http;
mod imp;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "basable=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app,
    )
    .await
    .unwrap();
}
