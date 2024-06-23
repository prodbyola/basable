use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;

use crate::base::config::ConnectionConfig;
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
    Json(config): Json<ConnectionConfig>,
) -> Result<Json<DbConnectionDetails>, AppError> {
    let mut bsbl = state.instance.lock().unwrap();

    let user_id = user_id.unwrap();
    // let mut auth_session = false;

    if let Some(user) = bsbl.find_user(&user_id) {
        let user = user.borrow();
        let auth_session = user.is_logged;
        drop(user);
        
        if auth_session {
            bsbl.save_config(&config, &user_id);
        }
        
        let (db, table_configs) = Basable::create_connection(&config, user_id)?;
        bsbl.add_connection(&db);
    
        // let mut user = user.borrow_mut();
        // user.init_table_configs(table_configs)?;
    
        let resp = db.details()?;
        return Ok(Json(resp))
    }

    Err(AppError::new(StatusCode::UNAUTHORIZED, "User unknown"))
    
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
