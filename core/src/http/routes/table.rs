use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{
    base::{table::TableConfig, AppError},
    http::{app::AppState, middlewares::AuthExtractor},
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
        let conn = &mut conn.lock().unwrap();

        let exists = conn.table_exists(&table_name)?;

        if !exists {
            let msg = format!("The '{}' table does not exist.", table_name);

            return Err(AppError::new(StatusCode::NOT_FOUND, &msg));
        }

        let table = conn.get_table(&table_name);

        if let Some(table) = table  {
            let mut table = table.lock().unwrap();
            table.save_config(config, !user.is_logged)?;
        }

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
) -> Result<Json<Option<TableConfig>>, AppError> {
    if let Some(user) = user {
        let bsbl = state.instance.lock().unwrap();
        let conn = bsbl.get_connection(&user.id).unwrap();
        let conn = conn.lock().unwrap();

        let exists = conn.table_exists(&table_name)?;

        if !exists {
            let msg = format!("The '{}' table does not exist.", table_name);

            return Err(AppError::new(StatusCode::NOT_FOUND, &msg));
        }

        let mut config = None;
        if let Some(table) = conn.get_table(&table_name) {
            let table = table.lock().unwrap();
            config = table.get_config(!user.is_logged)?;
        }
        

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
