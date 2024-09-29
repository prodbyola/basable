use graphs::graphs_routes;
use axum::routing::{get, post};
use axum::Router;

use crate::base::config::ConfigRaw;
use crate::base::data::table::TableSummaries;
use crate::base::foundation::Basable;
use crate::base::{AppError, AppState};
use crate::http::middlewares::AuthExtractor;
use crate::imp::database::DbServerDetails;
use axum::{extract::State, Json};
use axum_macros::debug_handler;

use self::auth::auth_routes;
use self::table::table_routes;

use super::middlewares::DbExtractor;

pub(super) mod auth;
pub(super) mod table;
pub(super) mod graphs;

#[debug_handler]
async fn connect(
    State(state): State<AppState>,
    AuthExtractor(user): AuthExtractor,
    Json(config): Json<ConfigRaw>,
) -> Result<Json<String>, AppError> {
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

    Ok(Json(db.id().to_string()))
}

#[debug_handler]
pub(crate) async fn table_summaries(
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<Json<TableSummaries>, AppError> {
    let summaries = db.query_table_summaries()?;

    Ok(Json(summaries))
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
        .route("/table-summaries", get(table_summaries))
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
