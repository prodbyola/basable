use core::str;
use std::sync::{Arc, Mutex};

use axum::http::StatusCode;
use data::table::TableConfig;
use foundation::Basable;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

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
            .map_err(|err| AppError::PersistentStorageError(err.to_string()))
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
        .map_err(|err| AppError::PersistentStorageError(err.to_string()))
    }

    pub fn create_table_config(&self, conn_id: &str, tc: TableConfig) -> Result<usize, AppError> {
        match self.pool() {
            Ok(pool) => {
                let exec = pool.execute(
                    "
                    INSERT INTO table_configs (conn_id, label, pk_column, name, ipp)
                    VALUES (?1, ?2, ?3, ?4, ?5)
                ",
                    params![conn_id, tc.label, tc.pk_column, tc.name, tc.items_per_page],
                );

                exec.map_err(|err| {
                    AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
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
    ) -> Result<usize, AppError> {
        match self.pool() {
            Ok(pool) => {
                let exec = pool.execute(
                    "UPDATE table_configs SET name = ?, label = ?, pk_column = ?, ipp = ?, WHERE name = ? AND conn_id = ?", 
                    params![tc.name, tc.label, tc.pk_column, tc.items_per_page, name, conn_id]
                );

                exec.map_err(|err| {
                    AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                })
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn get_table_config(&self, id: &str, conn_id: &str) -> Result<TableConfig, AppError> {
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
                    AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
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

impl AppState {
    pub fn create() -> Result<Self, AppError> {
        let manager = SqliteConnectionManager::memory();
        let pool = r2d2::Pool::new(manager).map_err(|err| AppError::InitError(err.to_string()))?;

        let i = Self {
            instance: Default::default(),
            local_db: LocalDB(pool),
        };

        Ok(i)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        tests::common::{create_test_db, create_test_instance},
        AppError,
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
