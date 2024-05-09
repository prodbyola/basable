use std::collections::HashMap;

use serde::Serialize;

pub(crate) mod mysql;

pub(crate) type DBVersion = HashMap<String, String>;
pub(crate) type TableSummaries = Vec<TableSummary>;

#[derive(Serialize)]
pub(crate) struct TableSummary {
    pub name: String,
    pub row_count: u32,
    pub col_count: u32,
    pub created: Option<String>,
    pub updated: Option<String>,
}

#[derive(Serialize, Default)]
pub(crate) struct DbConnectionDetails {
    pub tables: TableSummaries,
    pub version: DBVersion,
    pub db_size: f64,
}