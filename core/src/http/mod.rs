use std::collections::HashMap;

use axum::extract::Query;
use axum::{extract::State, Json};
use chrono::{Local, Utc};
use serde_json::json;

use crate::base::{Config, ObaseDB, RowCountOption};
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
    let tbn = params.get("table").unwrap();
    let col = params.get("created_at");

    let mut db = state.db.lock().unwrap();

    let mut tb = db.table(tbn);
    let rc = tb.row_count(None).unwrap();

    match col {
        Some(col) => {
            let date_column = String::from(col);
            let day = match params.get("day") {
                Some(d) => String::from(d),
                None => {
                    let utc = Utc::now();
                    let local = utc.with_timezone(&Local);
                    local.format("%Y-%m-%d").to_string()
                }
            };

            let opt = RowCountOption { 
                date: Some(day),
                date_column,
                date_selection: crate::base::CountDateSelection::Day
            };

            let count = tb.row_count(Some(opt)).unwrap();
            println!("count: {}", count);
        }
        None => {
            // Send a ws message indicating user didn't specify a 
            // `created_at` column.
        }
    }

    let data = json!({
        "row_count": rc
    });


    serde_json::to_string(&data).unwrap()
}