use std::net::SocketAddr;

use crate::base::auth::JwtSession;
use crate::base::config::Config;
use crate::base::foundation::{Basable, BasableConnection};
use crate::base::AppError;
use crate::imp::database::DatabaseConnectionDetails;
use crate::AppState;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use crate::http::middlewares::AuthExtractor;

pub(crate) mod middlewares;

#[debug_handler]
/// POST: /create-guest 
/// 
/// Creates a Basable guest `User` and returns a `JwtSession`.
pub(crate) async fn create_guest_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> Result<Json<JwtSession>, AppError> {
    let mut bsbl = state.instance.lock().unwrap();

    let addr = addr.ip().to_string();
    let session = bsbl.create_guest_user(&addr)?;

    Ok(Json(session))
}

#[debug_handler]
/// POST: /connect 
/// 
/// Creates a new `BasableConnection` for current user. It expects `Config` as request's body.
pub(crate) async fn connect(
    State(state): State<AppState>,
    AuthExtractor(user): AuthExtractor,
    Json(config): Json<Config>,
) -> Result<Json<DatabaseConnectionDetails>, AppError> {
    let mut resp = DatabaseConnectionDetails::default();
    let mut bsbl = state.instance.lock().unwrap();

    if let Some(user) = user {
        if user.is_logged {
            bsbl.save_config(&config, &user.id);
        }

        if let Some(conn) = Basable::create_connection(&config)? {
            bsbl.add_connection(user.id.clone(), conn);
            
            
            let conn = bsbl.get_connection(&user.id).unwrap().to_owned();
            let conn: std::sync::MutexGuard<'_, dyn BasableConnection<Error = AppError>> =
                conn.lock().unwrap();
    
            resp = conn.get_details()?;
        } 
    } else {
        return Err(AppError(StatusCode::NOT_FOUND, String::from("User not found! Please try to login again.")))
    }


    Ok(Json(resp))
}