use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    routing::{get, patch, post, put},
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{
    base::{
        column::ColumnList,
        table::{DataQueryFilter, Table, TableConfig, UpdateDataOptions},
        AppError, AppState,
    },
    http::middlewares::{AuthExtractor, DbExtractor, TableExtractor},
    imp::database::mysql::table::MySqlTable,
};

#[debug_handler]
pub(crate) async fn save_configuration(
    Path(table_name): Path<String>,
    AuthExtractor(user): AuthExtractor,
    DbExtractor(db): DbExtractor,
    TableExtractor(_): TableExtractor,
    State(_): State<AppState>,
    Json(config): Json<TableConfig>,
) -> Result<String, AppError> {
    let conn_id = db.id().to_string();
    user.update_table_config(&conn_id, &table_name, config);

    Ok("Operation successful".to_string())
}

#[debug_handler]
pub(crate) async fn get_configuration(
    Path(table_name): Path<String>,
    AuthExtractor(user): AuthExtractor,
    DbExtractor(db): DbExtractor,
    TableExtractor(_): TableExtractor,
    State(_): State<AppState>,
) -> Result<Json<Option<TableConfig>>, AppError> {
    let conn_id = db.id().to_string();
    let config = user.get_table_config(&conn_id, &table_name);

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
    DbExtractor(_): DbExtractor,
    TableExtractor(table): TableExtractor,
    State(_): State<AppState>,
) -> Result<Json<Vec<HashMap<String, <MySqlTable as Table>::ColumnValue>>>, AppError> {
    // TODO: Build query filter from url query params
    let filter = DataQueryFilter::default();
    let data = table.query_data(filter)?;

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
    Json(options): Json<UpdateDataOptions>,
) -> Result<String, AppError> {
    table.update_data(options)?;
    return Ok("Operation successful".to_string());
}

/// Routes for database table management
pub(super) fn table_routes() -> Router<AppState> {
    Router::new()
        .route("/configurations/:table_name", get(get_configuration))
        .route("/configurations/:table_name", put(save_configuration))
        .route("/columns/:table_name", get(get_columns))
        .route("/data/:table_name", get(query_data))
        .route("/data/:table_name", post(insert_data))
        .route("/data/:table_name", patch(update_data))
}
