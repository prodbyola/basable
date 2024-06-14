use axum::http::StatusCode;

use crate::base::{
    column::{Column, ColumnList},
    connector::Connector,
    table::{Table, TableConfig},
    AppError,
};

pub(crate) struct MySqlTable {
    pub name: String,
    pub config: Option<TableConfig>,
}

impl Table for MySqlTable {
    type Error = mysql::Error;
    type Row = mysql::Row;

    fn name(&self) -> &str {
        &self.name
    }

    /// Query all columns for the table
    fn query_columns(
        &self,
        conn: &dyn Connector<Error = Self::Error, Row = Self::Row>,
    ) -> Result<ColumnList, AppError> {
        let query = format!(
            "
                SELECT column_name, column_type, is_nullable, column_default 
                FROM information_schema.columns 
                WHERE table_name = '{}'
            ",
            self.name
        );

        match conn.exec_query(&query) {
            Ok(result) => {
                let cols: ColumnList = result
                    .iter()
                    .map(|r| {
                        let name: String = r.get("COLUMN_NAME").unwrap();
                        let col_type: String = r.get("COLUMN_TYPE").unwrap();
                        let default: Option<String> = r.get("COLUMN_DEFAULT").unwrap();

                        let nullable: Option<String> = r.get("IS_NULLABLE");
                        let nullable = nullable.map(|s| s == "YES".to_owned()).unwrap();

                        Column {
                            name,
                            col_type,
                            default,
                            nullable,
                        }
                    })
                    .collect();

                Ok(cols)
            }
            Err(e) => Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
            )),
        }
    }
}
