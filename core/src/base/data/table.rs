use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    base::query::{
        filter::{Filter, FilterChain},
        BasableQuery, QueryCommand, QueryOrder,
    },
    AppError,
};

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
    pub label: String,

    pub name: String,

    /// Name of column to use as primary key.
    pub pk_column: Option<String>,

    /// Total number of items to be loaded for each pagination
    pub items_per_page: usize,

    /// Column for querying when a row was created.
    pub created_column: Option<HistoryColumn>,

    /// Column for querying when a row was updated.
    pub updated_column: Option<HistoryColumn>,

    /// Special columns that return `SpecialValueType`
    pub special_columns: Option<Vec<SpecialColumn>>,

    /// Notification events for this table.
    pub events: Option<Vec<NotifyEvent>>,

    /// Columns to exclude from fetch query
    pub exclude_columns: Option<Vec<String>>,
}

impl PartialEq for TableConfig {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Default for TableConfig {
    fn default() -> Self {
        TableConfig {
            pk_column: None,
            label: String::new(),
            name: String::new(),
            items_per_page: 100,
            created_column: None,
            updated_column: None,
            special_columns: None,
            events: None,
            exclude_columns: None,
        }
    }
}

#[derive(Deserialize)]
pub struct TableQueryOpts {
    /// The table we're querying
    pub table: String,

    /// Query offset
    pub offset: usize,

    /// Query row count
    pub row_count: usize,

    /// Query filters
    pub filters: Option<Vec<Filter>>,

    /// The columns(s) you want selected in the query. If set to `None` all fields
    /// will be selected.
    pub columns: Option<Vec<String>>,

    pub order_by: Option<QueryOrder>,
    pub search_opts: Option<TableSearchOpts>
}

impl TableQueryOpts {
    pub fn is_search_mode(&self) -> bool {
        self.search_opts.is_some()
    }
}

impl TryFrom<TableQueryOpts> for BasableQuery {
    type Error = AppError;

    fn try_from(opts: TableQueryOpts) -> Result<Self, Self::Error> {
        let TableQueryOpts {
            table,
            offset,
            row_count,
            filters,
            columns,
            order_by,
            search_opts,
        } = opts;

        let operation = QueryCommand::SelectData(columns);
        let filter_chain = filters
            .map_or(FilterChain::empty(), |filters| {
                FilterChain::prefill(filters)
            });

        let bq = BasableQuery {
            table,
            command: operation,
            row_count: Some(row_count),
            offset: Some(offset),
            filters: filter_chain,
            order_by,
            search_opts,
            ..Default::default()
        };

        Ok(bq)
    }
}

#[derive(Deserialize)]
pub(crate) struct TableSearchOpts {
    pub search_cols: Vec<String>,
    pub query: String,
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
pub(crate) struct UpdateTableData {
    pub unique_key: String,
    pub columns: Vec<String>,
    pub unique_values: Vec<String>,
    pub input: Vec<HashMap<String, String>>,
}
