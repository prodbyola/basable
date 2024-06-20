use uuid::Uuid;

use crate::imp::database::DbConnectionDetails;

use super::{
    connector::Connector,
    table::{SharedTable, TableSummaries},
    AppError,
};

pub(crate) type DBQueryResult<R, E> = Result<Vec<R>, E>;

/// An abstraction of database connection.
pub(crate) trait DB: Send + Sync {
    type Row;
    type Error;
    type ColumnValue;

    /// Get the `DB`'s connector instance.
    fn connector(&self) -> &dyn Connector<Row = Self::Row, Error = Self::Error>;

    /// Get connection id
    fn get_id(&self) -> Uuid;

    /// Load available tables into `DB` instance. Caller should provide a `TableDB`,
    /// an atomic `DB` pointer for referencing the `Table` parent.
    fn load_tables(
        &mut self,
    ) -> Result<(), AppError>;

    /// Query `DB` server for information about available tables
    fn query_tables(&self) -> DBQueryResult<Self::Row, Self::Error>;

    /// Get an instance of a table with a given name. The return table is mutable across threads.
    fn get_table(
        &self,
        name: &str,
    ) -> Option<&SharedTable<Self::Error, Self::Row, Self::ColumnValue>>;

    /// Query connection tables from DB source and return table summaries
    fn query_table_summaries(&mut self) -> Result<TableSummaries, AppError>;

    /// Check if a table with the given name exists in the database connection.
    fn table_exists(&self, name: &str) -> Result<bool, AppError>;

    /// Details about the connection
    fn details(&mut self) -> Result<DbConnectionDetails, AppError>;

    /// Get total number of columns
    fn query_column_count(&self, table_name: &str) -> Result<u32, AppError>;
}
