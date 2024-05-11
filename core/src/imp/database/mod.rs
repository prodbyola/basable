use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::base::foundation::TableList;

pub(crate) mod mysql;

pub(crate) type DBVersion = HashMap<String, String>;

#[derive(Serialize, Default)]
pub(crate) struct DbConnectionDetails {
    pub tables: TableList,
    pub version: DBVersion,
    pub db_size: f64,
}

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
    Image, Audio, Video, PDF, Webpage
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
    Create, Update, Delete
}

#[derive(Deserialize, Serialize, Clone)]
/// When shoult `NotifyEvent` get triggered around `NotifyTrigger`.
pub(crate) enum NotifyTriggerTime {
    Before, After
}

#[derive(Deserialize, Serialize, Clone)]
/// The REST API method expected by the webhook URL.
pub(crate) enum NotifyEventMethod {
    Get, Post, Delete, Put, Patch
}

#[derive(Deserialize, Serialize, Clone)]
/// What should happen to the operation `NotifyTrigger` when there's notification error?
/// Let's say there's a server error from the webhook URL, should we proceed or fail the operation? 
pub(crate) enum OnNotifyError {
    Fail, Proceed
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
    events: Option<Vec<NotifyEvent>>
}
