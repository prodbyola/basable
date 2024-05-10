use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::put,
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{
    base::AppError,
    http::{app::AppState, middlewares::AuthExtractor},
    imp::database::TableConfig,
};

#[debug_handler]
async fn update_configuration(
    Path(table_name): Path<String>,
    AuthExtractor(user): AuthExtractor,
    State(state): State<AppState>,
    Json(config): Json<TableConfig>,
) -> Result<(), AppError> {
    if let Some(user) = user {
        let bsbl = state.instance.lock().unwrap();
        let conn = bsbl.get_connection(&user.id).unwrap();
        let mut conn = conn.lock().unwrap();

        let exists = conn.table_exists(&table_name)?;

        if !exists {
            let msg = format!("The '{}' table does not exist.", table_name);

            return Err(AppError::new(
                StatusCode::NOT_FOUND,
                &msg,
            ));
        }

        conn.save_table_config(&table_name, config, !user.is_logged)?;
    }

    Ok(())
}

pub(super) fn table_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/configurations/:table_name", 
            put(update_configuration)
        )
}
