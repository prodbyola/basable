use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{
    base::AppError,
    http::{app::AppState, middlewares::AuthExtractor},
    imp::database::TableConfig,
};

#[debug_handler]
async fn save_configuration(
    Path(table_name): Path<String>,
    AuthExtractor(user): AuthExtractor,
    State(state): State<AppState>,
    Json(config): Json<TableConfig>,
) -> Result<String, AppError> {
    if let Some(user) = user {
        let bsbl = state.instance.lock().unwrap();
        let conn = bsbl.get_connection(&user.id).unwrap();
        let mut conn = conn.lock().unwrap();

        let exists = conn.table_exists(&table_name)?;

        if !exists {
            let msg = format!("The '{}' table does not exist.", table_name);

            return Err(AppError::new(StatusCode::NOT_FOUND, &msg));
        }

        conn.save_table_config(&table_name, config, !user.is_logged)?;

        return Ok(String::from("Operation successful."));
    }

    Err(AppError::new(
        StatusCode::EXPECTATION_FAILED,
        "User not active.",
    ))
}

#[debug_handler]
async fn get_configuration(
    Path(table_name): Path<String>,
    AuthExtractor(user): AuthExtractor,
    State(state): State<AppState>,
) -> Result<Json<TableConfig>, AppError> {
    if let Some(user) = user {
        let bsbl = state.instance.lock().unwrap();
        let conn = bsbl.get_connection(&user.id).unwrap();
        let mut conn = conn.lock().unwrap();

        let exists = conn.table_exists(&table_name)?;

        if !exists {
            let msg = format!("The '{}' table does not exist.", table_name);

            return Err(AppError::new(StatusCode::NOT_FOUND, &msg));
        }

        let config = conn.get_table_config(&table_name, !user.is_logged)?;

        return Ok(Json(config));
    }

    Err(AppError::new(
        StatusCode::EXPECTATION_FAILED,
        "User not active.",
    ))
}

pub(super) fn table_routes() -> Router<AppState> {
    Router::new()
        .route("/configurations/:table_name", put(save_configuration))
        .route("/configurations/:table_name", get(get_configuration))
}
