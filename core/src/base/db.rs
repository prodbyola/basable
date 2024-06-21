use uuid::Uuid;

use crate::imp::database::DbConnectionDetails;

use super::{
    table::{SharedTable, TableConfigs, TableSummaries},
    AppError, ConnectorType,
};

pub(crate) type DBQueryResult<R, E> = Result<Vec<R>, E>;

/// An abstraction of database connection.
pub(crate) trait DB: Send + Sync {
    type Row;
    type Error;
    type ColumnValue;

    /// Get the `DB`'s connector instance.
    fn connector(&self) -> &ConnectorType;

    /// Get connection id
    fn get_id(&self) -> Uuid;

    /// Load available tables into `DB` instance. Caller should provide a [`ConnectorType`]
    /// pointer whose copy is assigned to each [Table](`crate::base::table::Table`) that is loaded. The [`ConnectorType`] will be
    /// used by the table for it's own queries.
    ///
    /// We also provide `load_table_configs` param, which which tells `load_tables` to attempt creating default configurations for [Table](`crate::base::table::Table`). Please
    /// see [Table::new](`crate::base::table::Table::new`) to learn more.
    fn load_tables(&mut self, connector: ConnectorType)
        -> Result<TableConfigs, AppError>;

    /// Query [`DB`] server for information about available tables. It only queries the database server and
    /// return results as [`DB::Row`]. It is different from [`DB::load_tables`] which actually loads the [`Table`]
    /// abstraction into memory.
    fn query_tables(&self) -> DBQueryResult<Self::Row, Self::Error>;

    /// Get an instance of a [`SharedTable`], as a mutable thread-safe reference.
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
