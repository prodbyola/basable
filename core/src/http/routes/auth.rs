use std::net::SocketAddr;

use axum::{extract::{ConnectInfo, State}, routing::post, Json, Router};
use axum_macros::debug_handler;

use crate::{base::{auth::JwtSession, AppError}, http::app::AppState};

#[debug_handler]
/// POST: /core/auth/guest 
/// 
/// Creates a Basable guest `User` and returns a `JwtSession`.
async fn create_guest_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> Result<Json<JwtSession>, AppError> {
    let mut bsbl = state.instance.lock().unwrap();

    let addr = addr.ip().to_string();
    let session = bsbl.create_guest_user(&addr)?;

    Ok(Json(session))
}

pub(super) fn auth_routes () -> Router<AppState> {
    Router::new()
        .route("/guest", post(create_guest_user))
}