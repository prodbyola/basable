use std::net::SocketAddr;

use axum::extract::ConnectInfo;
use axum::response::IntoResponse;
use axum::{extract::State, Json};
use axum_macros::debug_handler;
use crate::types::config::Config;
use crate::{AppState, AuthExtractor};

/// POST: Creates a new database connection. It expects `Config` as request's body.
#[debug_handler]
pub(crate) async fn connect(
    State(state): State<AppState>,
    AuthExtractor(user): AuthExtractor,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(config): Json<Config>,

) -> impl IntoResponse {

    let mut bsbl = state.instance.lock().unwrap();
    
    let user_id = match user {
        Some(u) => {
            bsbl.save_new_config(&config, &u.id);
            String::from(&u.id)
        },
        None => {
            let iddr = addr.ip().to_string();
            println!("addr: {}", addr);
            let sid = bsbl.create_guest_user(iddr, &config);
            sid
        },
    };

    let conn = bsbl.get_connection(&user_id).unwrap().to_owned();

    let mut conn = conn.lock().unwrap();
    let table_names = conn.table_names().unwrap();
    serde_json::to_string(&table_names).unwrap()

}

// pub(crate) async fn columns(
//     State(state): State<AppState>,
//     Query(params): Query<HashMap<String, String>>
// ) -> String {
//     let mut bsbl = state.instance.lock().unwrap();
//     let mut table = db.get_table(params.get("table").unwrap());

//     let cols = table.show_columns().unwrap();

//     serde_json::to_string(&cols).unwrap()
// }

// pub(crate) async fn dashboard(
//     State(state): State<AppState>,
//     Query(params): Query<HashMap<String, String>>
// ) -> String {
//     let tbn = params.get("table").unwrap();
//     let col = params.get("created_at");

//     let mut db = state.db.lock().unwrap();

//     let mut tb = db.get_table(tbn);
//     let rc = tb.row_count(None).unwrap();

//     match col {
//         Some(col) => {
//             let date_column = String::from(col);
//             let day = match params.get("day") {
//                 Some(d) => String::from(d),
//                 None => {
//                     let utc = Utc::now();
//                     let local = utc.with_timezone(&Local);
//                     local.format("%Y-%m-%d").to_string()
//                 }
//             };

//             let opt = RowCountOption { 
//                 date: Some(day),
//                 date_column,
//                 date_selection: crate::base::CountDateSelection::Day
//             };

//             let count = tb.row_count(Some(opt)).unwrap();
//             println!("count: {}", count);
//         }
//         None => {
//             // Send a ws message indicating user didn't specify a 
//             // `created_at` column.
//         }
//     }

//     let data = json!({
//         "row_count": rc
//     });


//     serde_json::to_string(&data).unwrap()
// }
