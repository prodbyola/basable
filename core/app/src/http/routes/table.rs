use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use axum_macros::debug_handler;
use base::mysql::ColumnValue;
use common::data::{
    columns::ColumnList,
    table::{
        TableConfig, TableExportOpts, TableExportResponse, TableQueryOpts, TableSummaries,
        UpdateTableData,
    },
};
use uuid::Uuid;

use crate::{
    http::middlewares::{AuthExtractor, DbExtractor, TableExtractor},
    state::AppState,
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
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
    Json(opts): Json<TableQueryOpts>,
) -> Result<Json<Vec<HashMap<String, ColumnValue>>>, AppError> {
    let data = table.query_data(opts, &db)?;
    Ok(Json(data))
}

#[debug_handler]
pub(crate) async fn query_result_count(
    Path(_): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
    Json(filter): Json<TableQueryOpts>,
) -> Result<Json<usize>, AppError> {
    let count = table.query_result_count(filter, &db)?;
    Ok(Json(count))
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
    match (params.get("column"), params.get("values")) {
        (Some(column), Some(values)) => {
            let values: Vec<&str> = values.split(",").collect();
            table.delete_data(column, values)?;
            Ok("Operation successful".to_string())
        }
        _ => Err(AppError::HttpError(
            StatusCode::EXPECTATION_FAILED,
            "query params 'column' and 'values' cannot be empty".to_string(),
        )),
    }
}

pub(crate) async fn export(
    Path(_): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
    Json(opts): Json<TableExportOpts>,
) -> Result<Json<TableExportResponse>, AppError> {
    let format = opts.format.clone();
    let data = table.export(opts, &db)?;
    let resp = TableExportResponse {
        data,
        mimetype: format.as_mimetype(),
        filename: format!("{}.{}", Uuid::new_v4(), format.as_extension()),
    };

    Ok(Json(resp))
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

#[debug_handler]
pub(crate) async fn clear_table(
    Path(_): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(_): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
) -> Result<String, AppError> {
    table.clear()?;
    Ok("operation successful".to_string())
}

#[debug_handler]
pub(crate) async fn drop_table(
    Path(table_name): Path<String>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<String, AppError> {
    db.drop_table(&table_name)?;
    Ok("operation successful".to_string())
}

/// Define routes for managing database table
pub(super) fn table_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(load_tables))
        .route("/configurations/:table_name", get(get_configuration))
        .route("/configurations/:table_name", patch(save_configuration))
        .route("/columns/:table_name", get(get_columns))
        .route("/query-data/:table_name", post(query_data))
        .route("/query-result-count/:table_name", post(query_result_count))
        .route("/data/:table_name", post(insert_data))
        .route("/data/:table_name", patch(update_data))
        .route("/data/:table_name", delete(delete_data))
        .route("/data/export/:table_name", post(export))
        .route("/data/clear/:table_name", delete(clear_table))
        .route("/drop/:table_name", delete(drop_table))
}
