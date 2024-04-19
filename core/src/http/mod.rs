use std::net::SocketAddr;

use crate::base::auth::JwtSession;
use crate::base::config::Config;
use crate::base::foundation::{Basable, BasableConnection, ConnectionDetails};
use crate::base::AppError;
use crate::{AppState, AuthExtractor};
use axum::extract::ConnectInfo;
use axum::{extract::State, Json};
use axum_macros::debug_handler;


/// POST: Creates a new database connection. It expects `Config` as request's body.
#[debug_handler]
pub(crate) async fn connect(
    State(state): State<AppState>,
    AuthExtractor(user): AuthExtractor,
    Json(config): Json<Config>,
) -> Result<Json<ConnectionDetails>, AppError> {
    let mut resp = ConnectionDetails::default();
    let mut bsbl = state.instance.lock().unwrap();

    if let Some(user) = user {
        if user.is_logged {
            bsbl.save_new_config(&config, &user.id);
        }

        if let Some(conn) = Basable::create_connection(&config)? {
            bsbl.add_connection(user.id.clone(), conn);
            
            
            let conn = bsbl.get_connection(&user.id).unwrap().to_owned();
            let conn: std::sync::MutexGuard<'_, dyn BasableConnection<Error = AppError>> =
                conn.lock().unwrap();
    
            resp = conn.get_details()?;
        }

    }

    Ok(Json(resp))
}

#[debug_handler]
pub(crate) async fn create_guest_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> Result<Json<JwtSession>, AppError> {
    let mut bsbl = state.instance.lock().unwrap();

    let addr = addr.ip().to_string();
    let session = bsbl.create_guest_user(&addr)?;

    Ok(Json(session))
}