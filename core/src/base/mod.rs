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

use crate::AppError;

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
    fn pool(&self) -> Result<PooledConnection<SqliteConnectionManager>, AppError> {
        self.0
            .get()
            .map_err(|err| AppError::PersistentStorageFailed(err.to_string()))
    }

    pub fn setup(&self) -> Result<usize, AppError> {
        let pool = self.pool()?;

        pool.execute(
            "CREATE TABLE IF NOT EXISTS table_configs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                conn_id TEXT NOT NULL,
                label TEXT,
                pk_column TEXT,
                ipp INTEGER
            )",
            params![],
        )
        .map_err(|err| AppError::PersistentStorageFailed(err.to_string()))
    }

    pub fn create_table_config(&self, conn_id: &str, tc: TableConfig) -> Result<usize, HttpError> {
        match self.pool() {
            Ok(pool) => {
                let exec = pool.execute(
                    "
                    INSERT INTO table_configs (conn_id, label, pk_column, name)
                    VALUES (?1, ?2, ?3, ?4)
                ",
                    params![conn_id, tc.label, tc.pk_column, tc.name],
                );

                exec.map_err(|err| {
                    HttpError::new(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
                })
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn update_table_config(
        &self,
        name: &str,
        conn_id: &str,
        tc: TableConfig,
    ) -> Result<usize, HttpError> {
        match self.pool() {
            Ok(pool) => {
                let exec = pool.execute(
                    "UPDATE table_configs SET name = ?, label = ?, pk_column = ?, ipp = ?, WHERE name = ? AND conn_id = ?", 
                    params![tc.name, tc.label, tc.pk_column, tc.items_per_page, name, conn_id]
                );

                exec.map_err(|err| {
                    HttpError::new(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
                })
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn get_table_config(&self, id: &str, conn_id: &str) -> Result<TableConfig, HttpError> {
        match self.pool() {
            Ok(pool) => {
                let tc = pool.query_row(
                    "SELECT name, label, pk_column, ipp FROM table_configs WHERE (name = ?1 OR label = ?1) AND conn_id = ?2 LIMIT 1",
                    params![id, conn_id],
                    |row| {
                        Ok(TableConfig {
                            name: row.get(0)?,
                            label: row.get(1)?,
                            pk_column: row.get(2)?,
                            items_per_page: row.get(3)?,
                            ..Default::default()
                        })
                    },
                );

                tc.map_err(|err| {
                    HttpError::new(StatusCode::INTERNAL_SERVER_ERROR, &err.to_string())
                })
            }
            Err(err) => Err(err.into()),
        }
    }
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub instance: Arc<Mutex<Basable>>,
    pub local_db: LocalDB,
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
