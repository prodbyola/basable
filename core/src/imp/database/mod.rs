use std::collections::HashMap;

use serde::Serialize;

use crate::base::table::TableList;

pub(crate) mod mysql;

pub(crate) type DBVersion = HashMap<String, String>;

#[derive(Serialize, Default)]
pub(crate) struct DbConnectionDetails {
    pub tables: TableList,
    pub version: DBVersion,
    pub db_size: f64,
}
