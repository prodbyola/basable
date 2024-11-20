use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;

use axum::body::Body;
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::http::{HeaderName, Response};
use axum::routing::get_service;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, MatchedPath, Request},
    http::{
        header::{ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE},
        request::Parts,
        HeaderValue, StatusCode,
    },
    Router,
};

use tower::service_fn;
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tower_http::services::ServeDir;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::base::AppState;
use crate::AppError;

use super::routes::core_routes;

type BasableHttpService = IntoMakeServiceWithConnectInfo<Router<()>, std::net::SocketAddr>;

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

pub fn app() -> Result<BasableHttpService, AppError> {
    let wl = "http://localhost:5173"
        .parse::<HeaderValue>()
        .map_err(|err| AppError::InitError(err.to_string()))?;
    // We add CORS middleware to enable connection from Vue/React Development client
    let cors = CorsLayer::new()
        .allow_origin(wl)
        .allow_headers([
            ACCEPT,
            ACCESS_CONTROL_ALLOW_HEADERS,
            CONTENT_TYPE,
            HeaderName::from_static("session-id"),
            HeaderName::from_static("connection-id"),
        ])
        .allow_methods(Any);

    let state = AppState::create()?;
    state.local_db.setup()?;

    let routes = core_routes();

    let r = Router::new()
        .nest_service("/web", get_service(ServeDir::new("./web")))
        .nest("/core", routes)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|req: &Request| {
                            let method = req.method();
                            let uri = req.uri();

                            let matched_path = req
                                .extensions()
                                .get::<MatchedPath>()
                                .map(|matched_path| matched_path.as_str());

                            tracing::debug_span!("request", %method, %uri, matched_path)
                        })
                        .on_failure(()),
                )
                .layer(cors),
        )
        .fallback_service(service_fn(serve_index_html))
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>();

    Ok(r)
}

// Serve `index.html` for unknown routes
// Serve `index.html` for unknown routes
async fn serve_index_html(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();

    // Check if the path matches a file in the `dist` directory
    let potential_file_path = format!("./{}", path);
    if tokio::fs::metadata(&potential_file_path).await.is_ok() {
        // Return 404 if a file was requested but not found
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap());
    }

    // Serve `index.html` for other cases
    let index_path = PathBuf::from("./web/index.html");
    match tokio::fs::read(index_path).await {
        Ok(file) => Ok(Response::builder()
            .header("Content-Type", "text/html")
            .body(Body::from(file))
            .unwrap()),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Error loading index.html"))
            .unwrap()),
    }
}
