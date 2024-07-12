use std::collections::HashMap;

use serde::Serialize;

use crate::base::data::table::TableSummaries;

pub(crate) mod mysql;

pub(crate) type DBVersion = HashMap<String, String>;

#[derive(Serialize, Default)]
pub(crate) struct DbConnectionDetails {
    pub id: String,
    pub tables: TableSummaries,
    pub version: DBVersion,
    pub db_size: f64,
}
