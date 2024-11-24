use common::{
    data::{columns::ColumnList, table::{DataQueryResult, TableConfig, TableExportOpts, TableQueryOpts, UpdateTableData}},
    error::AppError,
};
use std::collections::HashMap;

use crate::mysql_plugin::ColumnValue;

use super::{ConnectorType, SharedDB};

pub trait Table: TableCRUD + Sync + Send {
    type Row;

    /// Create a new [`Table`] and assign the given [`ConnectorType`].
    ///
    /// It returns the new [`Table`]. And if a [`TableConfig`] is created for the table,
    /// then the config is also returned. It is up to the caller to save or send the config for the table.
    ///
    /// # Example:
    /// ```
    /// let (table, config) = Table::new("table_name".to_string(), conn);
    /// // config is Option<TableConfig>
    /// ```
    ///
    /// This call initializes [`TableConfig`] for the table if certain query conditions are true for the table.
    /// For example if the table has a column named id, a primary key or a unique column, we automatically
    /// set the `pk` field of the table to any of the column.
    fn new(name: String, conn: ConnectorType) -> Self
    where
        Self: Sized;

    /// [Table]'s name
    fn name(&self) -> &str;

    /// Get the table's [`ConnectorType`].
    fn connector(&self) -> &ConnectorType;

    /// Retrieve available columns for the table and build a [`ColumnList`].
    fn query_columns(&self) -> Result<ColumnList, AppError>;

    /// Create table's initial [`TableConfig`] if possible. Caller is responsible for
    /// saving the configuration in persistent DB.
    fn init_config(&self) -> Option<TableConfig>;
}

pub trait TableCRUD {
    /// Inserts a new data into the table.
    fn insert_data(&self, input: HashMap<String, String>) -> Result<(), AppError>;

    /// Retrieve data from table based on query `filter`.
    fn query_data(
        &self,
        filter: TableQueryOpts,
        db: &SharedDB,
    ) -> DataQueryResult<ColumnValue, AppError>;

    /// Get total size of returnable data based on [TableQueryOpts].
    fn query_result_count(&self, filter: TableQueryOpts, db: &SharedDB) -> Result<usize, AppError>;

    fn update_data(&self, input: UpdateTableData) -> Result<(), AppError>;

    fn delete_data(&self, col: String, value: String) -> Result<(), AppError>;

    fn export(&self, opts: TableExportOpts, db: &SharedDB) -> Result<String, AppError>;
}