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
    AuthExtractor(user): AuthExtractor,
    Json(config): Json<ConnectionConfig>,
) -> Result<Json<DbConnectionDetails>, AppError> {
    let mut bsbl = state.instance.lock().unwrap();

    let user_id = user.id;
    let (db, table_configs) = Basable::create_connection(&config, user_id)?;

    bsbl.add_connection(&db);
    if let Some(cfs) = table_configs {
        let conn_id = db.id().to_string();
        bsbl.add_configs(conn_id, cfs);
    }

    let resp = db.details()?;
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
        tests::{common::{create_test_config, create_test_state}, extractors::auth_extractor},
    };

    use super::connect;

    #[tokio::test]
    async fn test_connect_route() -> Result<(), AppError> {
        let state = create_test_state(false)?;
        let config = create_test_config();

        let extractor = auth_extractor();

        let c = connect(State(state), extractor, Json(config)).await;
        assert!(c.is_ok());

        Ok(())
    }
}
