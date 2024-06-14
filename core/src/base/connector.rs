use std::fmt::Display;

use super::{config::Config, AppError};

/// Facilitates connection and run queries between `Basable` instance and a databse server
pub(crate) trait Connector: Send + Sync {
    type Row;
    type Error: Display;
    /// Create a new connector
    fn new(conn: Config) -> Result<Self, AppError>
    where
        Self: Sized;

    /// Execute a database query and return results
    fn exec_query(&self, query: &str) -> Result<Vec<Self::Row>, Self::Error>;
}