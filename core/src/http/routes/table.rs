use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{
    base::{
        column::ColumnList,
        data::table::{TableConfig, TableQueryOpts, TableSummaries, UpdateTableData},
        imp::{graphs::FromQueryParams, table::Table},
        AppState,
    },
    http::middlewares::{AuthExtractor, DbExtractor, TableExtractor},
    imp::database::mysql::table::MySqlTable,
    AppError,
};

#[debug_handler]
pub(crate) async fn save_configuration(
    Path(table_name): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    TableExtractor(_): TableExtractor,
    State(state): State<AppState>,
    Json(config): Json<TableConfig>,
) -> Result<String, AppError> {
    let storage = state.local_db;
    let conn_id = db.id().to_string();

    storage.update_table_config(&table_name, &conn_id, config)?;
    Ok("Operation successful".to_string())
}

#[debug_handler]
pub(crate) async fn get_configuration(
    Path(table_name): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    TableExtractor(_): TableExtractor,
    State(state): State<AppState>,
) -> Result<Json<TableConfig>, AppError> {
    let storage = state.local_db;
    let conn_id = db.id().to_string();

    let config = storage.get_table_config(&table_name, &conn_id)?;
    Ok(Json(config))
}

#[debug_handler]
pub(crate) async fn get_columns(
    Path(_): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(_): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
) -> Result<Json<ColumnList>, AppError> {
    let cols = table.query_columns()?;

    Ok(Json(cols))
}

#[debug_handler]
pub(crate) async fn query_data(
    Path(_): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
) -> Result<Json<Vec<HashMap<String, <MySqlTable as Table>::ColumnValue>>>, AppError> {
    let filter = TableQueryOpts::from_query_params(params)?;
    let data = table.query_data(filter, &db)?;

    Ok(Json(data))
}

#[debug_handler]
pub(crate) async fn insert_data(
    Path(_): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(_): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
    Json(data): Json<HashMap<String, String>>,
) -> Result<String, AppError> {
    table.insert_data(data)?;
    Ok("Operation successful".to_string())
}

#[debug_handler]
pub(crate) async fn update_data(
    Path(_): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(_): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
    Json(options): Json<UpdateTableData>,
) -> Result<String, AppError> {
    table.update_data(options)?;
    Ok("Operation successful".to_string())
}

pub(crate) async fn delete_data(
    Query(params): Query<HashMap<String, String>>,
    Path(_): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(_): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
) -> Result<String, AppError> {
    let col = params.get("col");
    let value = params.get("value");

    let err = |msg: &str| AppError::HttpError(StatusCode::EXPECTATION_FAILED, msg.to_string());

    match (col, value) {
        (None, None) => Err(err("Please provide filter 'col' and 'value' query params.")),
        (None, Some(_)) => Err(err("Please provide 'value' query param.")),
        (Some(_), None) => Err(err("Please provide 'col' query param.")),
        (Some(col), Some(value)) => {
            table.delete_data(col.clone(), value.clone())?;
            Ok("Operation successful".to_string())
        }
    }
}

#[debug_handler]
pub(crate) async fn load_tables(
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<Json<TableSummaries>, AppError> {
    let tables = db.build_table_list()?;

    Ok(Json(tables))
}

/// Routes for database table management
pub(super) fn table_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(load_tables))
        .route("/configurations/:table_name", get(get_configuration))
        .route("/configurations/:table_name", patch(save_configuration))
        .route("/columns/:table_name", get(get_columns))
        .route("/data/:table_name", get(query_data))
        .route("/data/:table_name", post(insert_data))
        .route("/data/:table_name", patch(update_data))
        .route("/data/:table_name", delete(delete_data))
}
