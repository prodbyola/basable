use axum::routing::{get, post};
use axum::Router;
use graphs::graphs_routes;

use crate::base::config::ConfigRaw;
use crate::base::foundation::Basable;
use crate::base::AppState;
use crate::http::middlewares::AuthExtractor;
use crate::imp::database::DbServerDetails;
use crate::AppError;
use axum::{extract::State, Json};
use axum_macros::debug_handler;

use self::auth::auth_routes;
use self::table::table_routes;

use super::middlewares::DbExtractor;

pub(super) mod auth;
pub(super) mod graphs;
pub(super) mod table;

#[debug_handler]
async fn connect(
    State(state): State<AppState>,
    AuthExtractor(user): AuthExtractor,
    Json(config): Json<ConfigRaw>,
) -> Result<Json<String>, AppError> {
    let mut bsbl = state.instance.lock().unwrap();
    let storage = state.local_db;

    let user_id = user.id.clone();
    let db = Basable::create_connection(&config, user_id)?;

    bsbl.add_connection(&db);
    std::mem::drop(bsbl); // release Mutex lock

    let conn_id = db.id().to_string();
    let tables = db.tables();
    if !tables.is_empty() {
        for tbl in tables {
            if let Some(config) = tbl.init_config() {
                storage.create_table_config(&conn_id, config)?;
            }
        }
    }

    Ok(Json(conn_id))
}

async fn server_details(
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<Json<DbServerDetails>, AppError> {
    let details = db.details()?;

    Ok(Json(details))
}

pub(super) fn core_routes() -> Router<AppState> {
    Router::new()
        .route("/connect", post(connect))
        .route("/server", get(server_details))
        .nest("/auth", auth_routes())
        .nest("/tables", table_routes())
        .nest("/graphs", graphs_routes())
}

#[cfg(test)]
mod tests {
    use axum::{extract::State, Json};

    use crate::{
        tests::{
            common::{create_test_config, create_test_state},
            extractors::auth_extractor,
        },
        AppError,
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
