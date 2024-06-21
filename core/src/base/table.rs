use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::base::column::ColumnList;

use super::{AppError, ConnectorType};

pub(crate) type SharedTable<E, R, C> = Arc<Mutex<dyn Table<Error = E, Row = R, ColumnValue = C>>>;

pub(crate) type TableSummaries = Vec<TableSummary>;
pub(crate) type TableConfigs = Option<Vec<TableConfig>>;

pub(crate) type DataQueryResult<V, E> = Result<Vec<HashMap<String, V>>, E>;

#[derive(Deserialize, Serialize, Clone)]
/// Table column used for querying table history such as when a row was added or when a row was updated.
pub(crate) struct HistoryColumn {
    name: String,
    format: String,
    has_time: bool,
}

#[derive(Deserialize, Serialize, Clone)]
/// The type of `SpecialColumn`
pub(crate) enum SpecialValueType {
    Image,
    Audio,
    Video,
    PDF,
    Webpage,
}

#[derive(Deserialize, Serialize, Clone)]
/// Special columns are columns whose values should lead to some sort of media types.
pub(crate) struct SpecialColumn {
    name: String,
    special_type: SpecialValueType,
    path: String,
}

#[derive(Deserialize, Serialize, Clone)]
/// The action that should trigger `NotifyEvent`.
enum NotifyTrigger {
    Create,
    Update,
    Delete,
}

#[derive(Deserialize, Serialize, Clone)]
/// When should `NotifyEvent` get triggered around `NotifyTrigger`.
pub(crate) enum NotifyTriggerTime {
    Before,
    After,
}

#[derive(Deserialize, Serialize, Clone)]
/// The REST API method expected by the webhook URL.
pub(crate) enum NotifyEventMethod {
    Get,
    Post,
    Delete,
    Put,
    Patch,
}

#[derive(Deserialize, Serialize, Clone)]
/// What should happen to the operation `NotifyTrigger` when there's notification error?
/// Let's say there's a server error from the webhook URL, should we proceed or fail the operation?
pub(crate) enum OnNotifyError {
    Fail,
    Proceed,
}

#[derive(Deserialize, Serialize, Clone)]
/// Event sent to a given webhook URL based on certain `NotifyTrigger`
pub(crate) struct NotifyEvent {
    trigger: NotifyTrigger,
    trigger_time: NotifyTriggerTime,
    method: NotifyEventMethod,
    url: String,
    on_error: OnNotifyError,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct TableConfig {
    pub table_id: String,

    /// Name of column to use as primary key.
    pub pk: Option<String>,

    /// Column for querying when a row was created.
    pub created_column: Option<HistoryColumn>,

    /// Column for querying when a row was updated.
    pub updated_column: Option<HistoryColumn>,

    /// Special columns that return `SpecialValueType`
    pub special_columns: Option<Vec<SpecialColumn>>,

    /// Notification events for this table.
    pub events: Option<Vec<NotifyEvent>>,
}

impl PartialEq for TableConfig {
    fn eq(&self, other: &Self) -> bool {
        self.table_id == other.table_id
    }
}

impl Default for TableConfig {
    fn default() -> Self {
        TableConfig {
            pk: None,
            table_id: String::new(),
            created_column: None,
            updated_column: None,
            special_columns: None,
            events: None,
        }
    }
}

pub struct DataQueryFilter {
    /// Query pagination
    pub limit: usize,

    /// Columns to exclude from query
    pub exclude: Option<Vec<String>>,
}

impl Default for DataQueryFilter {
    fn default() -> Self {
        DataQueryFilter {
            limit: 100,
            exclude: None,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct TableSummary {
    pub name: String,
    pub row_count: u32,
    pub col_count: u32,
    pub created: Option<String>,
    pub updated: Option<String>,
}

pub(crate) trait Table: Sync + Send {
    type Error;
    type Row;
    type ColumnValue;

    /// Create a new [`Table`] and assign the given [`ConnectorType`].
    ///
    /// If `load_table_configs` is true, the we try to build [`TableConfig`] for the [`Table`]
    /// using available properties from the DB server.
    fn new(name: String, conn: ConnectorType) -> (Self, Option<TableConfig>)
    where
        Self: Sized;

    /// [Table]'s name
    fn name(&self) -> &str;

    /// Retrieve all columns for the table
    fn query_columns(&self) -> Result<ColumnList, Self::Error>;

    /// Retrieve data from table based on query `filter`.
    fn query_data(
        &self,
        filter: DataQueryFilter,
    ) -> DataQueryResult<Self::ColumnValue, Self::Error>;

    /// Get the table's [`ConnectorType`].
    fn connector(&self) -> &ConnectorType;

    fn insert_data(&self, data: HashMap<String, String>) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{
        base::{table::DataQueryFilter, AppError},
        tests::common::{create_test_instance, get_test_db_table, get_test_user_id},
    };

    #[test]
    fn test_table_exist() -> Result<(), AppError> {
        let user_id = get_test_user_id();
        let bsbl = create_test_instance(true)?;

        let user = bsbl.find_user(&user_id);
        let user = user.unwrap().borrow();

        let db = user.db().unwrap();
        // let db_ref = db.clone();
        let db = db.lock().unwrap();

        let table_name = get_test_db_table();

        // let tt = Arc::new(Box::new(db_ref));
        // db.load_tables(db_ref)?;
        assert!(db.table_exists(&table_name)?);

        Ok(())
    }

    #[test]
    fn test_table_query_column() -> Result<(), AppError> {
        let user_id = get_test_user_id();
        let bsbl = create_test_instance(true)?;

        let user = bsbl.find_user(&user_id);
        let user = user.unwrap().borrow();

        let db = user.db();
        let db = db.unwrap();
        let db = db.lock().unwrap();

        let table_name = get_test_db_table();

        assert!(db.get_table(&table_name).is_some());

        if let Some(table) = db.get_table("swp") {
            let table = table.lock().unwrap();
            let cols = table.query_columns();

            assert!(cols.is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_table_query_data() -> Result<(), AppError> {
        let user_id = get_test_user_id();
        let bsbl = create_test_instance(true)?;

        let user = bsbl.find_user(&user_id);
        let user = user.unwrap().borrow();

        let db = user.db();
        let db = db.unwrap();
        let db = db.lock().unwrap();

        let table_name = get_test_db_table();

        if let Some(table) = db.get_table(&table_name) {
            let table = table.lock().unwrap();
            let filter = DataQueryFilter::default();
            let data = table.query_data(filter);
            assert!(data.is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_table_insert_data() -> Result<(), AppError> {
        let user_id = get_test_user_id();
        let bsbl = create_test_instance(true)?;

        let user = bsbl.find_user(&user_id);
        let user = user.unwrap().borrow();

        let db = user.db();
        let db = db.unwrap();
        let db = db.lock().unwrap();

        let table_name = get_test_db_table();

        if let Some(table) = db.get_table(&table_name) {
            let mut test_data = HashMap::new();
            test_data.insert("username".to_owned(), "toonfortdm".to_owned());
            test_data.insert("password".to_owned(), "anewpassword".to_owned());

            let table = table.lock().unwrap();
            let insert_data = table.insert_data(test_data);
            
            assert!(insert_data.is_ok());
        }

        Ok(())
    }

}
