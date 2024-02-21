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

    let table_names = &db.table_names();
    let mut cols = Vec::new();

    if !table_names.is_empty() {
        let mut table = db.table(table_names.first().unwrap());
        cols = table.show_columns().unwrap();
    }

    let data = json!({
        "tables": table_names,
        "columns": cols
    });

    serde_json::to_string(&data).unwrap()
}
