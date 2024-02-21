use axum::{extract::State, Json};
use serde_json::json;

use crate::base::{Config, ObaseDB};
use crate::AppState;

pub(crate) async fn connect(
    State(state): State<AppState>,
    Json(config): Json<Config>, 
) -> String {

    let mut db = state.db.lock().unwrap();
    let new_db = ObaseDB::new(config);

    *db = new_db;

    let tables = &db.table_names();

    let data = json!({
        "tables": tables
    });

    serde_json::to_string(&data).unwrap()
}
