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
use connector::Connector;
use db::DB;
use foundation::Basable;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;
use table::Table;

use crate::imp::database::mysql::{connector::MysqlConnector, db::MySqlDB, table::MySqlTable};

pub(crate) mod column;
pub(crate) mod config;
pub(crate) mod connector;
pub(crate) mod db;
pub(crate) mod foundation;
pub(crate) mod table;
pub(crate) mod user;

/// Dynamic [`DB`] type implemented across the app.
pub(crate) type DbType = dyn DB<
    Row = <MySqlDB as DB>::Row,
    Error = <MySqlDB as DB>::Error,
    ColumnValue = <MySqlDB as DB>::ColumnValue,
>;

/// Dynamic [`Connector`] type implemented across the app.
pub(crate) type ConnectorType = Arc<
    dyn Connector<
        Row = <MysqlConnector as Connector>::Row,
        Error = <MysqlConnector as Connector>::Error,
    >,
>;

/// Dynamic [`Table`] type implemented across the app.
pub(crate) type TableType = dyn Table<
    Row = <MySqlTable as Table>::Row,
    Error = <MySqlTable as Table>::Error,
    ColumnValue = <MySqlTable as Table>::ColumnValue,
>;

pub(crate) type SharedDB = Arc<DbType>;

#[derive(Clone)]
pub(crate) struct AppState {
    pub instance: Arc<Mutex<Basable>>,
    pub local_db: Pool<SqliteConnectionManager>,
}

impl AppState {
    fn pool(&self) -> PooledConnection<SqliteConnectionManager> {
        self.local_db.get().unwrap()
    }
    pub fn setup_local_db(&self) {
        let pool = self.pool();
        pool.execute(
            "CREATE TABLE IF NOT EXISTS table_configs (id INTEGER)",
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
            local_db: pool,
        }
    }
}

#[derive(Debug)]
pub struct AppError(pub StatusCode, pub String);

impl AppError {
    pub(crate) fn new(code: StatusCode, msg: &str) -> Self {
        AppError(code, String::from(msg))
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        (self.0, self.1).into_response()
    }
}

impl Serialize for AppError {
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
        base::AppError,
        tests::common::{create_test_db, create_test_instance},
    };

    #[test]
    fn test_instance_has_db() -> Result<(), AppError> {
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
