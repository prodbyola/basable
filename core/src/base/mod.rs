use core::str;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use data::table::TableConfig;
use foundation::Basable;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;

pub(crate) mod column;
pub(crate) mod config;
pub(crate) mod data;
pub(crate) mod foundation;
pub(crate) mod imp;
pub(crate) mod query;
pub(crate) mod user;

#[derive(Clone)]
pub(crate) struct LocalDB(pub Pool<SqliteConnectionManager>);

impl LocalDB {
    fn pool(&self) -> PooledConnection<SqliteConnectionManager> {
        self.0.get().unwrap()
    }

    pub fn create_table_config(&self, conn_id: &str, tc: TableConfig) -> Result<usize, HttpError> {
        let pool = self.pool();
        let exec = pool.execute(
            "
            INSERT INTO table_configs (conn_id, label, pk_column)
            VALUES (?1, ?2, ?3)
        ",
            params![conn_id, tc.label, tc.pk_column],
        );

        match exec {
            Ok(rows) => Ok(rows),
            Err(err) => Err(HttpError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &err.to_string(),
            )),
        }
    }
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub instance: Arc<Mutex<Basable>>,
    pub local_db: LocalDB,
}

impl AppState {
    pub fn setup_local_db(&self) {
        let pool = self.local_db.pool();
        pool.execute(
            "CREATE TABLE IF NOT EXISTS table_configs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                conn_id TEXT NOT NULL,
                label TEXT,
                pk_column TEXT,
            )",
            params![],
        )
        .unwrap();
    }
}

impl Default for AppState {
    fn default() -> Self {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::new(manager).unwrap();

        Self {
            instance: Default::default(),
            local_db: LocalDB(pool),
        }
    }
}

#[derive(Debug)]
pub struct HttpError(pub StatusCode, pub String);

impl HttpError {
    pub fn new(code: StatusCode, msg: &str) -> Self {
        HttpError(code, String::from(msg))
    }

    pub fn not_implemented() -> Self {
        Self::new(StatusCode::NOT_IMPLEMENTED, "feature not implemented")
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response<Body> {
        (self.0, self.1).into_response()
    }
}

impl Serialize for HttpError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Ok(serializer.collect_str(&self.1)?)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        base::HttpError,
        tests::common::{create_test_db, create_test_instance},
    };

    #[test]
    fn test_instance_has_db() -> Result<(), HttpError> {
        let db = create_test_db();
        assert!(db.is_ok());

        Ok(())
    }

    #[test]
    fn test_create_instance() {
        let bsbl = create_test_instance(true);
        assert!(bsbl.is_ok());
    }
}
