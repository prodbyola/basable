use std::collections::HashMap;

use crate::base::{
    column::{Column, ColumnList},
    data::table::{DataQueryFilter, DataQueryResult, TableConfig, UpdateDataOptions},
    imp::{table::{Table, TableColumn, TableError, TableCRUD}, ConnectorType},
};

use super::MySqlValue;

pub(crate) struct MySqlTable {
    pub name: String,
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
        MySqlTable {
            name,
            connector: conn,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    /// Query all columns for the table
    fn query_columns(&self) -> Result<ColumnList, Self::Error> {
        let query = format!(
            "
                SELECT 
                    cols.column_name,
                    cols.column_type,
                    cols.is_nullable,
                    cols.column_default,
                    IF(stats.index_name IS NOT NULL, 'YES', 'NO') AS IS_UNIQUE,
                    IF(kcus.constraint_name IS NOT NULL, 'YES', 'NO') AS IS_PRIMARY
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
                LEFT JOIN
                    information_schema.key_column_usage AS kcus
                ON
                    cols.table_name = kcus.table_name
                    AND cols.column_name = kcus.column_name
                    AND kcus.constraint_name = 'PRIMARY'
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
                let name: String = r.get("COLUMN_NAME").unwrap();
                let col_type: String = r.get("COLUMN_TYPE").unwrap();
                let default: Option<String> = r.get("COLUMN_DEFAULT").unwrap();

                let nullable: Option<String> = r.get("IS_NULLABLE");
                let nullable = nullable.map(|s| s == "YES".to_owned()).unwrap();

                let unique: Option<String> = r.get("IS_UNIQUE");
                let unique = unique.map(|s| s == "YES".to_owned()).unwrap();
                
                let primary: Option<String> = r.get("IS_PRIMARY");
                let primary = primary.map(|s| s == "YES".to_owned()).unwrap();

                Column {
                    name,
                    col_type,
                    default_value: default,
                    nullable,
                    unique,
                    primary
                }
            })
            .collect();

        Ok(cols)
    }

    fn connector(&self) -> &ConnectorType {
        &self.connector
    }

    
    fn init_config(&self) -> Option<TableConfig> {
        let mut config = None;

        if let Ok(cols) = self.query_columns() {
            let mut iter = cols.iter();
            let mut pk = iter.find(|c| c.primary);

            if let None = pk {
                pk = iter.find(|c| c.unique);
            }

            let pk = pk.map(|pk| pk.name.clone());
            
            let c = TableConfig {
                pk,
                table_id: self.name.clone(),
                ..TableConfig::default()
            };

            config = Some(c);
        }

        config
    }
}

impl TableCRUD for MySqlTable {
    fn query_data(
        &self,
        filter: DataQueryFilter,
    ) -> DataQueryResult<TableColumn, TableError> {
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
        let data: Vec<HashMap<String, TableColumn>> = result
            .iter()
            .map(|r| {
                let mut map: HashMap<String, TableColumn> = HashMap::new();

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

    fn insert_data(&self, input: HashMap<String, String>) -> Result<(), TableError> {
        let len = input.len();
        let mut data = HashMap::new();

        for (k, v) in input {
            data.insert(k, format!("'{}'", v));
        }

        let mut keys = Vec::with_capacity(len);
        let mut values = Vec::with_capacity(len);

        data.iter().for_each(|(k, v)| {
            keys.push(k.as_str());
            values.push(v.as_str());
        });

        let keys = keys.join(", ");
        let values = values.join(", ");

        let query = format!("INSERT INTO {} ({}) VALUES ({})", self.name, keys, values);
        let conn = self.connector();
        conn.exec_query(&query)?;

        Ok(())
    }

    fn update_data(&self, options: UpdateDataOptions) -> Result<(), TableError> {
        let UpdateDataOptions { key, value, input } = options;

        let data: Vec<String> = input
            .iter()
            .map(|(k, v)| format!("{} = '{}'", k, v))
            .collect();
        let data = data.join(", ");

        let query = format!(
            "UPDATE {} SET {} WHERE {} = '{}'",
            self.name, data, key, value
        );
        let conn = self.connector();
        conn.exec_query(&query)?;

        Ok(())
    }

    fn delete_data(&self, col: String, value: String) -> Result<(), TableError> {
        let query = format!("DELETE FROM {} WHERE {} = '{}'", self.name, col, value);
        let conn = self.connector();
        conn.exec_query(&query)?;

        Ok(())
    }

}