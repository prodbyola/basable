use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub(crate) type TableSummaries = Vec<TableSummary>;

pub(crate) type DataQueryResult<V, E> = Result<Vec<HashMap<String, V>>, E>;

/// Table column used for querying table history such as when a row was added or when a row was updated.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct HistoryColumn {
    name: String,
    pattern: String,
}

/// The type of `SpecialColumn`
#[derive(Deserialize, Serialize, Clone)]
pub(crate) enum SpecialValueType {
    Image,
    Audio,
    Video,
    PDF,
    Webpage,
}

/// Special columns are columns whose values should lead to some sort of media types.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct SpecialColumn {
    name: String,
    special_type: SpecialValueType,
    path: String,
}

/// The action that should trigger `NotifyEvent`.
#[derive(Deserialize, Serialize, Clone)]
enum NotifyTrigger {
    Create,
    Update,
    Delete,
}

/// When should `NotifyEvent` get triggered around `NotifyTrigger`.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) enum NotifyTriggerTime {
    Before,
    After,
}

/// The REST API method expected by the webhook URL.
#[derive(Deserialize, Serialize, Clone)]
pub(crate) enum NotifyEventMethod {
    Get,
    Post,
    Delete,
    Put,
    Patch,
}

/// What should happen to the operation `NotifyTrigger` when there's notification error?
/// Let's say there's a server error from the webhook URL, should we proceed or fail the operation?
#[derive(Deserialize, Serialize, Clone)]
pub(crate) enum OnNotifyError {
    Fail,
    Proceed,
}

/// Event sent to a given webhook URL based on certain `NotifyTrigger`
#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Default)]
pub(crate) struct UpdateDataOptions {
    pub key: String,
    pub value: String,
    pub input: HashMap<String, String>,
}
