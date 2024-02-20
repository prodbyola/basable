use axum::{extract::State, Json};

use crate::base::{Config, ObaseDB};
use crate::AppState;

pub(crate) async fn login(
    State(state): State<AppState>,
    Json(config): Json<Config>, 
) -> String {

    let mut db = state.db.lock().unwrap();
    let new_db = ObaseDB::new(config);

    *db = new_db;

    serde_json::to_string(&db.table_names()).unwrap()
}
