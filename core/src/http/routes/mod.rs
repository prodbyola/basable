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

    let user_id = user_id.unwrap();

    if let Some(user) = bsbl.find_user(&user_id) {
        let user = user.lock().unwrap();

        if user.is_logged {
            bsbl.save_config(&config, &user_id);
        }

        // drop User MutexGuard from memory to prevent dreadlock when we try to
        // access the user instance later (for example, when we call `attach_db`).
        std::mem::drop(user);

        if let Some(conn) = Basable::create_connection(&config)? {
            bsbl.attach_db(&user_id, conn)?;

            let user = bsbl.find_user(&user_id).unwrap();
            let user = user.lock().unwrap();

            let conn = user.db().unwrap();
            let mut conn = conn.lock().unwrap();

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

#[cfg(test)]
mod tests {
    use axum::{extract::State, Json};

    use crate::{
        base::AppError,
        tests::common::{create_test_config, create_test_state, get_test_auth_extractor},
    };

    use super::connect;

    #[tokio::test]
    async fn test_connect_route() -> Result<(), AppError> {
        let state = create_test_state(false)?;
        let config = create_test_config();

        let extractor = get_test_auth_extractor();

        let c = connect(State(state), extractor, Json(config)).await;
        assert!(c.is_ok());

        Ok(())
    }
}
