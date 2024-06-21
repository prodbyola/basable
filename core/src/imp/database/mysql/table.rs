use std::collections::HashMap;

use crate::base::{
    column::{Column, ColumnList},
    table::{DataQueryFilter, DataQueryResult, Table, TableConfig},
    ConnectorType,
};

use super::MySqlValue;

pub(crate) struct MySqlTable {
    pub name: String,
    pub config: Option<TableConfig>,
    pub connector: ConnectorType,
}

impl Table for MySqlTable {
    type Error = mysql::Error;
    type Row = mysql::Row;
    type ColumnValue = MySqlValue;

    fn new(name: String, conn: ConnectorType) -> Self
    where
        Self: Sized,
    {
        let table = MySqlTable {
            name,
            connector: conn,
            config: None,
        };
        let mut config = None;

        if let Ok(cols) = table.query_columns() {
            let mut iter = cols.iter();
            let mut pk = iter.find(|c| c.name == "id");

            if let None = pk {
                pk = iter.find(|c| c.unique);
            }

            let pk = pk.map(|pk| pk.name.clone());
            let c = TableConfig {
                pk,
                ..TableConfig::default()
            };

            config = Some(c);
        }

        if let Some(c) = config {
            table.save_config(c, true).unwrap();
        }

        table
    }

    fn name(&self) -> &str {
        &self.name
    }

    /// Query all columns for the table
    fn query_columns(
        &self,
        // conn: &dyn Connector<Error = Self::Error, Row = Self::Row>,
    ) -> Result<ColumnList, Self::Error> {
        let query = format!(
            "
                SELECT 
                    cols.column_name,
                    cols.column_type,
                    cols.is_nullable,
                    cols.column_default,
                    IF(stats.index_name IS NOT NULL, 'YES', 'NO') AS IS_UNIQUE
                FROM 
                    information_schema.columns AS cols
                LEFT JOIN 
                    (SELECT DISTINCT
                        column_name,
                        index_name
                    FROM
                        information_schema.statistics
                    WHERE
                        table_name = '{}'
                        AND non_unique = 0) AS stats
                ON 
                    cols.column_name = stats.column_name
                    AND cols.table_name = '{}'
                WHERE
                    cols.table_name = '{}'

            ",
            self.name, self.name, self.name
        );

        let conn = self.connector();
        let result = conn.exec_query(&query)?;

        let cols: ColumnList = result
            .iter()
            .map(|r| {
                println!("{:?}", r);
                let name: String = r.get("COLUMN_NAME").unwrap();
                let col_type: String = r.get("COLUMN_TYPE").unwrap();
                let default: Option<String> = r.get("COLUMN_DEFAULT").unwrap();

                let nullable: Option<String> = r.get("IS_NULLABLE");
                let nullable = nullable.map(|s| s == "YES".to_owned()).unwrap();
                
                let unique: Option<String> = r.get("IS_UNIQUE");
                let unique = unique.map(|s| s == "YES".to_owned()).unwrap();

                Column {
                    name,
                    col_type,
                    default_value: default,
                    nullable,
                    unique,
                }
            })
            .collect();

        Ok(cols)
    }

    fn query_data(
        &self,
        filter: DataQueryFilter,
    ) -> DataQueryResult<Self::ColumnValue, Self::Error> {
        let cols = self.query_columns()?;
        let mut excluded_cols: Vec<&Column> = vec![]; // columns to exclude from query

        if let Some(exclude) = filter.exclude {
            if !exclude.is_empty() {
                excluded_cols = cols
                    .iter()
                    .filter(|col| {
                        let m = exclude.iter().find(|ex| col.name == **ex);
                        m.is_some()
                    })
                    .collect();
            }
        }

        // TODO: filter excluded columns from the query
        let query = format!("SELECT * FROM {} LIMIT {}", self.name(), filter.limit);
        let conn = self.connector();
        let result = conn.exec_query(&query)?;

        // std::mem::drop(db);
        let data: Vec<HashMap<String, Self::ColumnValue>> = result
            .iter()
            .map(|r| {
                let mut map: HashMap<String, Self::ColumnValue> = HashMap::new();

                for col in &cols {
                    if let None = excluded_cols.iter().find(|c| c.name == col.name) {
                        let v: mysql::Value = r.get(col.name.as_str()).unwrap();

                        map.insert(col.name.clone(), v.into());
                    }
                }

                map
            })
            .collect();

        Ok(data)
    }
    fn connector(&self) -> &ConnectorType {
        &self.connector
    }
}
