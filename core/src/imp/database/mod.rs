use std::collections::HashMap;

use serde::Serialize;

pub(crate) mod mysql;

pub(crate) type DBVersion = HashMap<String, String>;

#[derive(Serialize, Default)]
pub(crate) struct DbServerDetails {
    pub version: String,
    pub db_size: f64,
    pub os: String,
    pub comment: Option<String>
}
