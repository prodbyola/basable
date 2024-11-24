use std::collections::HashMap;

use serde::Serialize;

pub mod error;
pub mod data;
pub mod query;

pub type DBVersion = HashMap<String, String>;

#[derive(Serialize, Default)]
pub struct DbServerDetails {
    pub version: String,
    pub db_size: f64,
    pub os: String,
    pub comment: Option<String>
}