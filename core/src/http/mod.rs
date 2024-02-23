use std::collections::HashMap;

use axum::extract::Query;
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

    let table_names = &db.table_names().unwrap();

    serde_json::to_string(&table_names).unwrap()
}

pub(crate) async fn columns(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> String {
    let mut db = state.db.lock().unwrap();
    let mut table = db.table(params.get("table").unwrap());

    let cols = table.show_columns().unwrap();

    serde_json::to_string(&cols).unwrap()
}

pub(crate) async fn dashboard(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> String {
    let mut db = state.db.lock().unwrap();
    let mut table = db.table(params.get("table").unwrap());

    let count = table.count().unwrap();

    let data = json!({
        "count": count
    });


    serde_json::to_string(&data).unwrap()
}