use std::collections::HashMap;

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    base::{
        imp::graphs::FromQueryParams,
        query::{
            filter::{Filter, FilterChain},
            BasableQuery, QueryOperation,
        },
    },
    globals::DEFAULT_ROWS_PER_PAGE,
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
    pub exclude_columns: Option<Vec<String>>
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
            exclude_columns: None
        }
    }
}

pub struct TableQueryOpts {
    /// The table we're querying
    pub table: String,

    /// Query offset
    pub offset: usize,

    /// Query row count
    pub row_count: usize,

    /// Query filters
    pub filters: Option<Vec<String>>,

    /// The columns(s) you want selected in the query. If set to `None` all fields
    /// will be selected.
    pub columns: Option<Vec<String>>,
}

impl FromQueryParams for TableQueryOpts {
    fn from_query_params(params: HashMap<String, String>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let table = params.get("table");
        let row_count = params.get("row_count");
        let offset = params.get("offset");
        let filters = params.get("filters");
        let columns = params.get("columns");

        match table {
            Some(table) => {
                let row_count = match row_count {
                    Some(c) => c.parse::<usize>().map_err(|err| {
                        AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                    })?,
                    None => DEFAULT_ROWS_PER_PAGE,
                };

                let offset = match offset {
                    Some(c) => c.parse::<usize>().map_err(|err| {
                        AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                    })?,
                    None => 0,
                };

                let filters: Option<Vec<String>> =
                    filters.map(|s| s.split(",").map(|s| s.to_string()).collect());

                let columns: Option<Vec<String>> =
                    columns.map(|s| s.split(",").map(|s| s.to_string()).collect());

                let tqf = TableQueryOpts {
                    table: table.to_string(),
                    row_count,
                    offset,
                    filters,
                    columns,
                };

                Ok(tqf)
            }
            None => {
                let err = AppError::HttpError(
                    StatusCode::EXPECTATION_FAILED,
                    "Table name must be provided".to_string(),
                );

                Err(err)
            }
        }
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
        } = opts;

        let operation = QueryOperation::SelectData(columns);
        let mut filter_chain = FilterChain::new();
        if let Some(filters) = filters {
            for s in filters {
                let f = Filter::try_from(s).map_err(|err| {
                    AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                })?;
                filter_chain.add_one(f);
            }
        }

        let bq = BasableQuery {
            table,
            operation,
            row_count: Some(row_count),
            offset: Some(offset),
            filters: filter_chain,
            ..Default::default()
        };

        Ok(bq)
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
pub(crate) struct UpdateTableData {
    pub unique_key: String,
    pub columns: Vec<String>,
    pub unique_values: Vec<String>,
    pub input: Vec<HashMap<String, String>>,
}
