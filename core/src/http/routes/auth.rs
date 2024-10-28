use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    routing::post,
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{
    base::{foundation::Basable, user::JwtSession, AppState},
    AppError,
};

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

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use axum::extract::{ConnectInfo, State};

    use crate::{
        http::routes::auth::create_guest_user, tests::common::create_test_state, AppError,
    };

    #[tokio::test]
    async fn test_create_guest() -> Result<(), AppError> {
        let state = create_test_state(false)?;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

        let create_guest = create_guest_user(ConnectInfo(addr), State(state)).await;

        assert!(create_guest.is_ok());

        Ok(())
    }
}
