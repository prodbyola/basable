use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    routing::post,
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{foundation::Basable, state::AppState, user::JwtSession, AppError};

#[debug_handler]
async fn create_guest_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(_): State<AppState>,
) -> Result<Json<JwtSession>, AppError> {
    let addr = addr.ip().to_string();
    let session = Basable::create_guest_user(&addr)?;

    Ok(Json(session))
}

/// Routes for user session management and authentication
pub(super) fn auth_routes() -> Router<AppState> {
    Router::new().route("/guest", post(create_guest_user))
}