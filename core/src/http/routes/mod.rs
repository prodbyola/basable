use graphs::graphs_routes;
use axum::routing::post;
use axum::Router;

use crate::base::config::ConnectionConfig;
use crate::base::foundation::Basable;
use crate::base::{AppError, AppState};
use crate::http::middlewares::AuthExtractor;
use crate::imp::database::DbConnectionDetails;
use axum::{extract::State, Json};
use axum_macros::debug_handler;

use self::auth::auth_routes;
use self::table::table_routes;

pub(super) mod auth;
pub(super) mod table;
pub(super) mod graphs;

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

    let user_id = user.id.clone();
    let db = Basable::create_connection(&config, user_id)?;

    bsbl.add_connection(&db);
    std::mem::drop(bsbl); // release Mutex lock

    let tables = db.tables();
    if !tables.is_empty() {
        tables.iter().for_each(|tbl| {
            if let Some(config) = tbl.init_config() {
                // TODO: Save table config to local db
            }
        })
    }

    let resp = db.details()?;
    Ok(Json(resp))
}

pub(super) fn core_routes() -> Router<AppState> {
    Router::new()
        .route("/connect", post(connect))
        .nest("/auth", auth_routes())
        .nest("/tables", table_routes())
        .nest("/graphs", graphs_routes())
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
