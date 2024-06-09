use axum::routing::post;
use axum::Router;

use crate::base::config::Config;
use crate::base::foundation::Basable;
use crate::base::AppError;
use crate::http::app::AppState;
use crate::http::middlewares::AuthExtractor;
use crate::imp::database::DbConnectionDetails;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use axum_macros::debug_handler;

use self::auth::auth_routes;
use self::table::table_routes;

pub(super) mod auth;
pub(super) mod table;

#[debug_handler]
/// POST: /core/connect
///
/// Creates a new `BasableConnection` for current user. It expects `Config` as request's body.
async fn connect(
    State(state): State<AppState>,
    AuthExtractor(user_id): AuthExtractor,
    Json(config): Json<Config>,
) -> Result<Json<DbConnectionDetails>, AppError> {
    let mut resp = DbConnectionDetails::default();
    let mut bsbl = state.instance.lock().unwrap();

    if let Some(user) = bsbl.find_user(&user_id.unwrap_or_default()) {
        let user = user.lock().unwrap();
        let user_id = user.id.clone();

        if user.is_logged {
            bsbl.save_config(&config, &user_id);
        }

        if let Some(conn) = Basable::create_connection(&config)? {
            bsbl.attach_db(&user_id, conn);

            let conn = user.db().unwrap();
            let mut conn =  conn.lock().unwrap();

            resp = conn.details()?;
        }
    } else {
        return Err(AppError::new(
            StatusCode::NOT_FOUND,
            "User not found! Please try to login again.",
        ));
    }

    Ok(Json(resp))
}

pub(super) fn core_routes() -> Router<AppState> {
    Router::new()
        .route("/connect", post(connect))
        .nest("/auth", auth_routes())
        .nest("/tables", table_routes())
}
