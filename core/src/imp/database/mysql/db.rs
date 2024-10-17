use std::{collections::HashMap, sync::Arc};

use mysql::Row;
use time::Date;
use uuid::Uuid;

use crate::{
    base::{
        config::ConfigRaw,
        data::table::{TableSummaries, TableSummary},
        imp::{
            db::{QuerySqlParser, DB},
            table::Table,
            ConnectorType, SharedTable,
        },
        HttpError,
    },
    imp::database::{DBVersion, DbServerDetails},
};

use super::{table::MySqlTable, MySqlValue};

pub(crate) struct MySqlDB {
    pub connector: ConnectorType,
    pub tables: Vec<SharedTable>,
    user_id: String,
    id: Uuid,
}

impl MySqlDB {
    pub fn new(connector: ConnectorType, user_id: String) -> Self {
        MySqlDB {
            connector,
            tables: Vec::new(),
            user_id,
            id: Uuid::new_v4(),
        }
    }

    /// Get MySQL server version and host OS version
    fn show_version_variables(&self) -> Result<DBVersion, HttpError> {
        let vars = self.exec_query(
            "
                SHOW VARIABLES 
                WHERE Variable_name 
                IN (
                    'version_compile_os', 
                    'version', 
                    'version_comment', 
                    'version_compile_zlib'
                )
            ",
        )?;

        let mut data = HashMap::new();

        for v in vars {
            let name: String = v.get("Variable_name").unwrap();
            let value: String = v.get("Value").unwrap();
            data.insert(name, value);
        }

        Ok(data)
    }

    fn size(&self) -> Result<f64, HttpError> {
        let db = self.config().db_name.as_ref().unwrap();

        let query = format!(
            "
            SELECT table_schema '{db}', 
            ROUND(SUM(data_length + index_length) / 1024 / 1024, 1) 'size' 
            FROM information_schema.tables 
            WHERE table_schema = '{db}'
            GROUP BY table_schema
        "
        );

        let qr = self.exec_query(&query)?;

        // db size is returned in MB, we may want to write a function
        // to convert for GB, TB...etc
        let size: f64 = qr.first().map_or(0.0, |r| {
            let s: String = r.get("size").unwrap();
            s.parse().unwrap()
        });

        Ok(size)
    }

    fn config(&self) -> &ConfigRaw {
        &self.connector.config()
    }

    fn exec_query(&self, query: &str) -> mysql::Result<Vec<Row>> {
        self.connector.exec_query(query)
    }
}

impl DB for MySqlDB {
    type Error = mysql::Error;
    type Row = mysql::Row;
    type ColumnValue = MySqlValue;

    fn id(&self) -> &Uuid {
        &self.id
    }

    fn user_id(&self) -> &str {
        &self.user_id
    }

    fn connector(&self) -> &ConnectorType {
        &self.connector
    }

    fn load_tables(&mut self, connector: ConnectorType) -> Result<(), HttpError> {
        let tables = self.query_tables()?;

        if !tables.is_empty() {
            tables.iter().for_each(|t| {
                let connector = connector.clone();
                let name: String = t.get("TABLE_NAME").unwrap();

                let table = MySqlTable::new(name, connector);
                self.tables.push(Arc::new(table));
            })
        }

        Ok(())
    }

    fn tables(&self) -> &Vec<SharedTable> {
        &self.tables
    }

    fn query_tables(&self) -> mysql::Result<Vec<Row>> {
        let query = format!(
            "
                SELECT table_name, table_rows, create_time, update_time
                FROM information_schema.tables
                WHERE table_schema = '{}'
                ORDER BY table_name;
            ",
            self.config().db_name.clone().unwrap()
        );

        self.connector.exec_query(&query)
    }

    fn build_table_list(&self) -> Result<TableSummaries, HttpError> {
        let results = self.query_tables()?;
        let tables: Vec<TableSummary> = results
            .iter()
            .map(|res| {
                let created = res.get("CREATE_TIME") as Option<Date>;
                let updated = res.get("CREATE_TIME") as Option<Date>;
                let name: String = res.get("TABLE_NAME").unwrap();

                let col_count = self.query_column_count(&name).unwrap();

                TableSummary {
                    name,
                    col_count,
                    row_count: res.get("TABLE_ROWS").unwrap(),
                    created: created.map_or(None, |d| Some(d.to_string())),
                    updated: updated.map_or(None, |d| Some(d.to_string())),
                }
            })
            .collect();

        Ok(tables)
    }

    fn query_column_count(&self, tb_name: &str) -> Result<u32, HttpError> {
        let query = format!(
            "
                SELECT count(*) 
                FROM information_schema.columns 
                WHERE table_schema = '{}' and table_name = '{}'
                ORDER BY table_name;
            ",
            self.config().db_name.clone().unwrap(),
            tb_name
        );

        let qr = self.exec_query(&query)?;
        let c: u32 = qr.first().map_or(0, |r| r.get("count(*)").unwrap());

        Ok(c)
    }

    fn get_table(&self, name: &str) -> Option<&SharedTable> {
        self.tables.iter().find(|t| t.name() == name)
    }

    fn details(&self) -> Result<DbServerDetails, HttpError> {
        let vrbs = self.show_version_variables()?;
        let version = vrbs.get("version").map(|v| v.to_string());
        let os = vrbs.get("version_compile_os").map(|v| v.to_string());
        let comment = vrbs.get("version_comment").map(|v| v.to_string());

        let size = self.size()?;

        Ok(DbServerDetails {
            version: version.unwrap_or_default(),
            os: os.unwrap_or_default(),
            comment,
            db_size: size,
        })
    }
}

impl QuerySqlParser for MySqlDB {}
