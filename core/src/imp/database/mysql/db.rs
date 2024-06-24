use std::{
    cell::RefCell, collections::HashMap, sync::Arc
};

use mysql::Row;
use time::Date;
use uuid::Uuid;

use crate::{
    base::{
        config::ConnectionConfig,
        db::DB,
        table::{SharedTable, Table, TableConfigList, TableSummaries, TableSummary},
        AppError, ConnectorType,
    },
    imp::database::{DBVersion, DbConnectionDetails},
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
            id: Uuid::new_v4()
        }
    }

    /// Get MySQL server version and host OS version
    fn show_version(&self) -> Result<DBVersion, AppError> {
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

    fn size(&self) -> Result<f64, AppError> {
        let db = self.config().db_name.as_ref().unwrap();

        let query = format!(
            "
                SELECT table_schema '{}', 
                ROUND(SUM(data_length + index_length) / 1024 / 1024, 1) 'size' 
                FROM information_schema.tables 
                WHERE table_schema = '{}'
                GROUP BY table_schema
            ",
            db, db
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

    fn config(&self) -> &ConnectionConfig {
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

    fn load_tables(
        &mut self,
        connector: ConnectorType,
    ) -> Result<Option<TableConfigList>, AppError> {
        let tables = self.query_tables()?;
        let mut configs = Vec::with_capacity(tables.len());

        if !tables.is_empty() {
            tables.iter().for_each(|t| {
                let connector = connector.clone();
                let name: String = t.get("TABLE_NAME").unwrap();
                let (table, config) = MySqlTable::new(name, connector);
                if let Some(config) = config {
                    configs.push(RefCell::new(config));
                }

                self.tables.push(Arc::new(table));
            })
        }

        let configs = if configs.is_empty() {
            None
        } else {
            Some(configs)
        };

        Ok(configs)
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

    fn query_table_summaries(&self) -> Result<TableSummaries, AppError> {
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

    fn table_exists(&self, name: &str) -> Result<bool, AppError> {
        let q = format!(
            "
                SELECT count(*) 
                FROM information_schema.tables
                WHERE table_schema = '{}' AND table_name = '{}'
            ",
            self.config().db_name.clone().unwrap(),
            name
        );

        let qr = self.exec_query(&q)?;
        let exists = qr.first().map_or(false, |r| r.get("count(*)").unwrap());

        Ok(exists)
    }

    fn query_column_count(&self, tb_name: &str) -> Result<u32, AppError> {
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

    fn get_table(
        &self,
        name: &str,
    ) -> Option<&SharedTable> {
        self.tables
            .iter()
            .find(|t| t.name() == name)
    }

    fn details(&self) -> Result<DbConnectionDetails, AppError> {
        let version = self.show_version()?;
        let tables = self.query_table_summaries()?;
        let size = self.size()?;
        let id = self.id.clone();
        let id = id.to_string();

        Ok(DbConnectionDetails {
            id,
            tables,
            version,
            db_size: size,
        })
    }
}
