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
use mysql::Row;
use table::SharedTable;
use uuid::Uuid;

use crate::imp::database::DbConnectionDetails;

use self::{config::Config, table::TableSummaries};

pub(crate) mod auth;
pub(crate) mod config;
pub(crate) mod foundation;
pub(crate) mod table;

/// A sharable connection that belongs to a specific user
type SharedConnection = Arc<Mutex<dyn DB>>;

/// Facilitates connection and run queries between `Basable` instance and a databse server
pub(crate) trait Connector: Send + Sync {
    /// Create a new connector
    fn new(conn: Config) -> Result<Self, AppError>
    where
        Self: Sized;

    /// Execute a database query and return results
    fn exec_query(&self, query: &str) -> mysql::Result<Vec<Row>>;
}

/// An abstraction of database connection.
pub(crate) trait DB: Send + Sync {
    /// Get the `DB`'s connector instance.
    fn connector(&self) -> &dyn Connector;
    
    /// Get connection id
    fn get_id(&self) -> Uuid;

    /// Load available tables into `DB` instance
    fn load_tables(&mut self) -> Result<(), AppError>;

    /// Query DB server for available tables
    fn query_tables(&self) -> mysql::Result<Vec<Row>>;

    /// Query connection tables from DB source and return table summaries
    fn query_table_summaries(&mut self) -> Result<TableSummaries, AppError>;

    /// Check if a table with the given name exists in the database connection.
    fn table_exists(&self, name: &str) -> Result<bool, AppError>;

    /// Get an instance of a table with a given name. The return table is mutable across threads.
    fn get_table(&self, name: &str) -> Option<&SharedTable>;

    /// Details about the connection
    fn details(&mut self) -> Result<DbConnectionDetails, AppError>;

    fn query_column_count(&self, table_name: &str) -> Result<u32, AppError>;
}

#[derive(Debug)]
pub(crate) struct AppError(pub StatusCode, pub String);

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

/// Implements conversion of `mysql::Error` to AppError. At the moment, all variations
/// of `mysql::Error` resolves to `StatusCode::INTERNAL_SERVER_ERROR`.
impl From<mysql::Error> for AppError {
    fn from(value: mysql::Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        (self.0, self.1).into_response()
    }
}

#[cfg(test)]
mod test {
    use crate::base::{foundation::Basable, AppError};

    use super::{auth::User, Config};

    static TEST_USER_ID: &str = "test_user";

    fn create_instance() -> Result<Basable, AppError> {
        let db_name = "basable";
        let mut config = Config::default();

        let mut user = User::default();
        user.id = TEST_USER_ID.to_owned();

        config.db_name = Some(String::from(db_name));
        config.username = Some(String::from(db_name));
        config.password = Some(String::from("Basable@2024"));
        config.host = Some(String::from("localhost"));
        config.port = Some(3306);

        let mut bslb = Basable::default();
        bslb.add_user(user);
        
        let conn = Basable::create_connection(&config)?;
        bslb.attach_db(TEST_USER_ID, conn.unwrap())?;

        Ok(bslb)
    }

    #[test]
    fn test_instance() -> Result<(), AppError> {
        let bsbl = create_instance()?;

        let user = bsbl.find_user(TEST_USER_ID);
        assert!(user.is_some());

        let user = user.unwrap();
        let user = user.lock().unwrap();

        assert!(user.db().is_some());

        let db = user.db();
        let db = db.unwrap();
        let mut db = db.lock().unwrap();

        db.load_tables()?;
        db.table_exists("swp")?;

        if let Some(table) = db.get_table("swp") {
            let table = table.lock().unwrap();
            table.get_columns(db.connector())?;
        }

        Ok(())
    }
}
