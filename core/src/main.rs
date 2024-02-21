use std::sync::{Arc, Mutex};

use axum::{http::{header::{ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE}, HeaderValue}, routing::post, Router};
use base::ObaseDB;
use http::login;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower::ServiceBuilder;

mod base;
mod http;

#[derive(Clone)]
pub(crate) struct AppState {
    db: Arc<Mutex<ObaseDB>>,
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(ObaseDB::default()));
    let state = AppState { db };

    // We created CORS middleware to enable connection from Vue Development server
    let cors =
        CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_headers([ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE]);

    let app = Router::new()
        .route("/app/connect", post(login))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
