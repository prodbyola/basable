use chrono::NaiveDateTime;
use serde::Serialize;

pub(crate) enum ColumnType {
    Text(String),
    Int(u32),
    DateTime(NaiveDateTime),
}

#[derive(Serialize)]
pub(crate) struct Column {
    pub name: String,
    pub col_type: String,
    pub nullable: bool,
    pub default: Option<String>,
}

pub(crate) type ColumnList = Vec<Column>;