use serde::Serialize;

#[derive(Serialize)]
pub struct Column {
    pub name: String,
    pub col_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub unique: bool,
    pub primary: bool
}

pub type ColumnList = Vec<Column>;