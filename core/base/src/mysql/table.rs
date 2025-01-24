use std::collections::HashMap;
use common::{data::{columns::{Column, ColumnList}, table::{DataQueryResult, TableConfig, TableExportFormat, TableExportOpts, TableQueryOpts, UpdateTableData}}, error::AppError, query::{filter::FilterChain, BasableQuery, QueryCommand}};

use crate::{table::{Table, TableCRUD}, ConnectorType, SharedDB};

use super::ColumnValue;

pub struct MySqlTable {
    pub name: String,
    pub connector: ConnectorType,
}
impl MySqlTable {
    fn search_index_name(&self, search_cols: &Vec<String>) -> String {
        let name = format!("bsearch_{}", search_cols.join("_"));
        name.replace(" ", "_")
    }

    fn create_search_index(&self, search_cols: &Vec<String>) -> Result<(), AppError> {
        let wrap_cols: Vec<String> = search_cols.iter().map(|col| format!("`{col}`")).collect();

        let index_name = self.search_index_name(&search_cols);
        let index_query = format!(
            "CREATE FULLTEXT INDEX {index_name} 
                ON {} ({})",
            self.name,
            wrap_cols.join(", ")
        );

        let conn = self.connector();
        conn.exec_query(&index_query)?;

        Ok(())
    }

    fn search_index_exists(&self, search_cols: &Vec<String>) -> bool {
        let index_name = self.search_index_name(&search_cols);

        let index_query = format!("SHOW INDEX FROM {}", self.name);
        let conn = self.connector();

        if let Ok(rows) = conn.exec_query(&index_query) {
            let exists = rows.iter().find(|row| {
                if let Some(name) = row.get::<String, &str>("Key_name") {
                    return name == index_name;
                }

                false
            });

            return exists.is_some();
        }

        false
    }

    fn drop_search_index(&self, search_cols: &Vec<String>) -> Result<(), AppError> {
        if self.search_index_exists(&search_cols) {
            let index_name = self.search_index_name(&search_cols);
            let index_query = format!("DROP INDEX {index_name} ON {};", self.name,);

            let conn = self.connector();
            conn.exec_query(&index_query)?;
        }

        Ok(())
    }

    fn search_prelude(&self, search_cols: &Vec<String>) -> Result<(), AppError> {
        self.drop_search_index(&search_cols)?;
        self.create_search_index(&search_cols)?;
        Ok(())
    }
}

impl Table for MySqlTable {
    type Row = mysql::Row;

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
    ) -> DataQueryResult<ColumnValue, AppError> {
        let is_search_mode = opts.is_search_mode();
        let mut search_cols = Vec::new();

        if is_search_mode {
            if let Some(search_opts) = &opts.search_opts {
                search_cols = search_opts.search_cols.clone();
                self.search_prelude(&search_cols)?;
            }
        }

        let cols = opts
            .columns
            .clone()
            .take_if(|cols| !cols.is_empty())
            .unwrap_or_else(|| match self.query_columns() {
                Ok(cs) => cs.iter().map(|col| col.name.clone()).collect(),
                Err(err) => {
                    tracing::error!("error reading db column: {err}");
                    vec![]
                }
            });

        let query = opts.try_into()?;
        let sql = db.generate_sql(query)?;

        let conn = self.connector();
        let rows = conn.exec_query(&sql)?;

        let data = rows
            .iter()
            .map(|r| {
                let mut map: HashMap<String, ColumnValue> = HashMap::new();

                for col in &cols {
                    if let Some(v) = r.get::<mysql::Value, &str>(col) {
                        map.insert(col.clone(), v.into());
                    }
                }

                map
            })
            .collect();

        if is_search_mode {
            let _ = self.drop_search_index(&search_cols)?;
        }

        Ok(data)
    }

    fn query_result_count(&self, opts: TableQueryOpts, db: &SharedDB) -> Result<usize, AppError> {
        let is_search_mode = opts.is_search_mode();
        let mut search_cols = Vec::new();

        if is_search_mode {
            if let Some(search_opts) = &opts.search_opts {
                search_cols = search_opts.search_cols.clone();
                self.search_prelude(&search_cols)?;
            }
        }

        let query = BasableQuery {
            table: opts.table,
            command: QueryCommand::SelectData(Some(vec!["COUNT(*)".to_string()])),
            search_opts: opts.search_opts,
            filters: opts
                .filters
                .map_or(FilterChain::empty(), |fs| FilterChain::prefill(fs)),
            ..Default::default()
        };

        let sql = db.generate_sql(query)?;

        let conn = self.connector();
        let rows = conn.exec_query(&sql)?;

        let count = rows
            .first()
            .map(|row| row.get::<usize, &str>("COUNT(*)").unwrap_or_default())
            .unwrap_or_default();

        if is_search_mode {
            let _ = self.drop_search_index(&search_cols)?;
        }

        Ok(count)
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

    fn delete_data(&self, col: &str, values: Vec<&str>) -> Result<(), AppError> {
        let query_prefix = format!("DELETE FROM {} WHERE", self.name);
        let query_suffix = if values.len() == 1 {
            match values.first() {
                Some(value) => Ok(format!("{col} = '{value}'")),
                None => Err(AppError::ServerError("a value must be provided".to_string()))
            }?
        } else {
            let vl = values.join(", ");
            format!("{col} IN ({vl})")
        };

        let query = format!("{query_prefix} {query_suffix}");
        let conn = self.connector();
        conn.exec_query(&query)?;

        Ok(())
    }

    fn export(&self, opts: TableExportOpts, db: &SharedDB) -> Result<String, AppError> {
        let TableExportOpts {
            query_opts,
            format,
            trim,
        } = opts;

        let cols = query_opts
            .columns
            .clone()
            .take_if(|cols| !cols.is_empty())
            .unwrap_or_else(|| match self.query_columns() {
                Ok(cs) => cs.iter().map(|col| col.name.clone()).collect(),
                Err(err) => {
                    tracing::error!("error reading db column: {err}");
                    vec![]
                }
            });

        let selection = if cols.is_empty() {
            None
        } else {
            Some(cols.clone())
        };

        let filters = query_opts.filters.map_or(FilterChain::empty(), |filters| {
            FilterChain::prefill(filters)
        });

        // get rows
        let query = BasableQuery {
            table: query_opts.table,
            command: QueryCommand::SelectData(selection),
            filters,
            offset: trim.as_ref().map_or(None, |trim| Some(trim.offset)),
            row_count: trim.map_or(None, |trim| Some(trim.count)),
            ..Default::default()
        };

        let sql = db.generate_sql(query)?;

        let conn = self.connector();
        let rows = conn.exec_query(&sql)?;
        let content = process_exports(format, cols, rows);

        Ok(content)
    }
    
    fn clear(&self) -> Result<(), AppError> {
        let query = format!("DELETE FROM {}", self.name);
        let conn = self.connector();
        conn.exec_query(&query)?;

        Ok(())
    }
}

fn process_exports(
    format: TableExportFormat,
    columns: Vec<String>,
    rows: Vec<mysql::Row>,
) -> String {
    match format {
        TableExportFormat::CSV
        | TableExportFormat::PSV
        | TableExportFormat::TSV
        | TableExportFormat::TEXT => {
            let delimiter = format.field_delimiter().unwrap_or_default();
            let headers = columns.join(&delimiter);
            let row_list: Vec<String> = rows
                .iter()
                .map(|row| {
                    let mut row_data = Vec::with_capacity(columns.len());
                    for col in &columns {
                        row_data.push(row.get::<String, &str>(col).unwrap_or_default());
                    }

                    row_data.join(&delimiter)
                })
                .collect();

            let body = row_list.join("\n");
            format!("{headers} \n {body}")
        }
        TableExportFormat::JSON => {
            let row_list: Vec<String> = rows
                .iter()
                .map(|row| {
                    let values: Vec<String> = columns
                        .iter()
                        .map(|col| {
                            let val = row.get::<String, &str>(col).unwrap_or_default();
                            format!("\t\"{}\":\"{}\"", col, val)
                        })
                        .collect();

                    format!("{{\n\t{}\n\t}}", values.join(",\n\t"))
                })
                .collect();

            format!("[\n\t{}\n]", row_list.join(",\n\t"))
        }
        TableExportFormat::HTML => {
            let col_list: Vec<String> = columns
                .iter()
                .map(|col| format!("<th>{col}</th>"))
                .collect();
            let header = format!(
                "<thead>\n\t\t<tr>\n\t\t\t{}\n\t\t</tr>\n\t</thead>",
                col_list.join("\n\t\t\t")
            );

            let row_list: Vec<String> = rows
                .iter()
                .map(|row| {
                    let td_list: Vec<String> = columns
                        .iter()
                        .map(|col| {
                            let val = row.get::<String, &str>(col).unwrap_or_default();
                            format!("<td>{val}</td>")
                        })
                        .collect();

                    format!("<tr>\n\t\t\t{}\n\t\t</tr>", td_list.join("\n\t\t\t"))
                })
                .collect();
            let body = format!("<tbody>\n\t\t{}\n\t</tbody>", row_list.join("\n\t\t"));

            format!("<table>\n\t{header}\n\t{body}\n<table>")
        }
        _ => "".to_string(),
    }
}
