use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Column {
    pub name: String,
    pub col_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub unique: bool
}

pub(crate) type ColumnList = Vec<Column>;