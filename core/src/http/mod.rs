use std::net::SocketAddr;

use crate::base::auth::JwtSession;
use crate::base::config::Config;
use crate::base::foundation::{BasableConnection, ConnectionDetails};
use crate::base::AppError;
use crate::{AppState, AuthExtractor};
use axum::extract::ConnectInfo;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use serde::Serialize;

#[derive(Default, Serialize)]
pub(crate) struct ConnectionResponse {
    session: Option<JwtSession>,
    details: ConnectionDetails,
}

/// POST: Creates a new database connection. It expects `Config` as request's body.
#[debug_handler]
pub(crate) async fn connect(
    State(state): State<AppState>,
    AuthExtractor(user): AuthExtractor,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(config): Json<Config>,
) -> Result<Json<ConnectionResponse>, AppError> {
    let mut resp = ConnectionResponse::default();
    let mut bsbl = state.instance.lock().unwrap();

    let user_id = match user {
        Some(u) => {
            if u.is_logged {
                bsbl.save_new_config(&config, &u.id);
            }

            String::from(&u.id)
        }
        None => {
            let iddr = addr.ip().to_string();

            let session = bsbl
                .create_guest_user(&iddr, &config)
                .expect("Unable to create user");
            
            resp.session = Some(session);

            iddr
        }
    };

    let conn = bsbl.get_connection(&user_id).unwrap().to_owned();

    let conn: std::sync::MutexGuard<'_, dyn BasableConnection<Error = AppError>> =
        conn.lock().unwrap();
    resp.details = conn.get_details()?;

    Ok(Json(resp))
}
