use std::{fmt::Display, sync::{Arc, Mutex}};

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::base::column::{Column, ColumnList};

use super::{connector::Connector, AppError};

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
    /// Column for querying when a row was created.
    created_column: Option<HistoryColumn>,

    /// Column for querying when a row was updated.
    updated_column: Option<HistoryColumn>,

    /// Special columns that return `SpecialValueType`
    special_columns: Option<Vec<SpecialColumn>>,

    /// Notification events for this table.
    events: Option<Vec<NotifyEvent>>,
}

pub(crate) type TableSummaries = Vec<TableSummary>;
// pub(crate) type SharedTable = Arc<Mutex<impl Table>>;
// pub(crate) type TableList = Vec<SharedTable>;

#[derive(Serialize)]
pub(crate) struct TableSummary {
    pub name: String,
    pub row_count: u32,
    pub col_count: u32,
    pub created: Option<String>,
    pub updated: Option<String>,
}

pub(crate) trait Table: Send + Sync {
    type Error;
    type Row;

    fn save_config(&self, config: TableConfig, save_local: bool) -> Result<(), AppError> {
        if save_local {
            // TODO: Save locally
        } else {
            // TODO: Save to remote server
        }

        Ok(())
    }

    fn get_config(&self, get_local: bool) -> Result<Option<TableConfig>, AppError> {
        if get_local {
            // TODO: Get locally
            Ok(None)
        } else {
            // TODO: Get from remote server
            return Err(AppError::new(
                StatusCode::NOT_IMPLEMENTED,
                "Not implemented",
            ));
        }
    }

    fn name(&self) -> &str;
    fn query_columns(&self, conn: &dyn Connector<Error = Self::Error, Row = Self::Row>) -> Result<ColumnList, AppError>;
}
