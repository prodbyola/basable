use axum::routing::post;
use axum::Router;

use crate::base::config::Config;
use crate::base::foundation::Basable;
use crate::base::AppError;
use crate::http::app::AppState;
use crate::http::middlewares::AuthExtractor;
use crate::imp::database::DbConnectionDetails;
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
    let mut bsbl = state.instance.lock().unwrap();

    let user_id = user_id.unwrap();

    if let Some(user) = bsbl.find_user(&user_id) {
        let user = user.borrow();
        let is_logged = user.is_logged;

        // drop User reference from memory in order to free `Basable` instance
        // and allow access later.
        std::mem::drop(user);

        if is_logged {
            bsbl.save_config(&config, &user_id);
        }
    }
    
    let db = Basable::create_connection(&config)?;
    bsbl.attach_db(&user_id, db)?;

    let user = bsbl.find_user(&user_id).unwrap();
    let user = user.borrow();

    let conn = user.db().unwrap();
    let mut conn = conn.lock().unwrap();

    let resp = conn.details()?;
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
        tests::common::{create_test_config, create_test_state, create_test_auth_extractor},
    };

    use super::connect;

    #[tokio::test]
    async fn test_connect_route() -> Result<(), AppError> {
        let state = create_test_state(false)?;
        let config = create_test_config();

        let extractor = create_test_auth_extractor();

        let c = connect(State(state), extractor, Json(config)).await;
        assert!(c.is_ok());

        Ok(())
    }
}
