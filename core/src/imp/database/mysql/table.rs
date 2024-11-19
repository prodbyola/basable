use std::collections::HashMap;

use crate::{
    base::{
        column::{Column, ColumnList},
        data::table::{DataQueryResult, TableConfig, TableQueryOpts, UpdateTableData},
        imp::{
            table::{Table, TableCRUD, TableColumn},
            ConnectorType, SharedDB,
        },
    },
    AppError,
};

use super::MySqlValue;

pub(crate) struct MySqlTable {
    pub name: String,
    pub connector: ConnectorType,
}

impl Table for MySqlTable {
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

    fn query_columns(&self) -> Result<ColumnList, AppError> {
        let table_name = &self.name;

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
                    table_name = '{table_name}'
                    AND non_unique = 0) AS stats
            ON 
                cols.column_name = stats.column_name
                AND cols.table_name = '{table_name}'
            LEFT JOIN
                information_schema.key_column_usage AS kcus
            ON
                cols.table_name = kcus.table_name
                AND cols.column_name = kcus.column_name
                AND kcus.constraint_name = 'PRIMARY'
            WHERE
                cols.table_name = '{table_name}'

        "
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
                    primary,
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
                pk_column: pk,
                name: self.name.clone(),
                label: self.name.clone(),
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
        opts: TableQueryOpts,
        db: &SharedDB,
    ) -> DataQueryResult<TableColumn, AppError> {
        let query = opts.try_into()?;
        let sql = db.generate_sql(query)?;

        println!("sql {sql}");

        let conn = self.connector();
        let rows = conn.exec_query(&sql)?;

        let cols = self.query_columns()?;
        let data = rows
            .iter()
            .map(|r| {
                let mut map: HashMap<String, TableColumn> = HashMap::new();

                for col in &cols {
                    if let Some(v) = r.get::<mysql::Value, &str>(col.name.as_str()) {
                        map.insert(col.name.clone(), v.into());
                    }
                }

                map
            })
            .collect();

        Ok(data)
    }

    fn insert_data(&self, input: HashMap<String, String>) -> Result<(), AppError> {
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

    fn update_data(&self, options: UpdateTableData) -> Result<(), AppError> {
        let UpdateTableData {
            unique_key,
            columns,
            unique_values,
            input,
        } = options;

        let mut cases = vec![];
        for (index, col) in columns.iter().enumerate() {
            let cmd = if index == 0 { "SET \n" } else { "" };
            let mut q = format!("{cmd} `{col}` = CASE `{unique_key}` \n");

            for (index, uv) in unique_values.iter().enumerate() {
                if let Some(values) = input.get(index) {
                    if let Some(val) = values.get(col) {
                        q.push_str(&format!("WHEN {uv} THEN '{val}' \n"));
                    }
                }
            }

            q.push_str(&format!("ELSE `{col}` \n END"));
            cases.push(q);
        }

        let cases = cases.join(", \n");
        let unique_values = unique_values.join(",");

        let query = format!(
            "UPDATE {} \n {} WHERE `{}` IN ({})",
            self.name, cases, unique_key, unique_values
        );

        let conn = self.connector();
        conn.exec_query(&query)?;

        Ok(())
    }

    fn delete_data(&self, col: String, value: String) -> Result<(), AppError> {
        let query = format!("DELETE FROM {} WHERE {} = '{}'", self.name, col, value);
        let conn = self.connector();
        conn.exec_query(&query)?;

        Ok(())
    }
}
