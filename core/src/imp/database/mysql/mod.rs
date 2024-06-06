use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use mysql::Result;
use mysql::{prelude::Queryable, Opts, Params, Pool, Row};
use time::Date;
use uuid::Uuid;

use crate::base::config::Config;
use crate::base::BasableConnection;
use crate::base::table::{SharedTable, Table, TableList, TableSummary};
use crate::base::AppError;

use super::{DBVersion, DbConnectionDetails, TableSummaries};

/// MySQL implementation of `BasableConnection`
#[derive(Clone, Default)]
pub struct MysqlConn {
    id: Uuid,
    user_id: String,

    /// An abstraction of database connection table list
    tables: TableList,

    /// Database connection pool
    pool: Option<Pool>,

    /// Connection options
    config: Config,

    // table_configs: Option<HashMap<String, TableConfig>>,
}

impl MysqlConn {
    fn pool(&self) -> Pool {
        self.pool.clone().unwrap()
    }

    fn exec_query(&self, query: &str) -> Result<Vec<Row>> {
        let conn = &mut self.pool().get_conn()?;

        let stmt = conn.prep(query)?;
        conn.exec(stmt, Params::Empty)
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

    fn get_column_count(&self, tb_name: &str) -> Result<u32, AppError> {
        let query = format!(
            "
                SELECT count(*) 
                FROM information_schema.columns 
                WHERE table_schema = '{}' and table_name = '{}'
                ORDER BY table_name;
            ",
            self.config.db_name.clone().unwrap(),
            tb_name
        );

        let qr = self.exec_query(&query)?;
        let c: u32 = qr.first().map_or(0, |r| r.get("count(*)").unwrap());

        Ok(c)
    }

    fn db_size(&self) -> Result<f64, AppError> {
        let db = self.config.db_name.as_ref().unwrap();

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
}

impl BasableConnection for MysqlConn {
    type Error = AppError;

    fn new(config: Config, user_id: &str) -> Result<Self, AppError> {
        let url = config.build_url();
        let opts = Opts::from_url(&url).unwrap();
        let pool = Pool::new(opts)?;

        Ok(MysqlConn {
            id: Uuid::new_v4(),
            user_id: String::from(user_id),
            pool: Some(pool),
            config,
            tables: Vec::new()
        })
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_user_id(&self) -> &str {
        &self.user_id
    }

    fn details(&mut self) -> Result<DbConnectionDetails, AppError> {
        let version = self.show_version()?;
        let tables = self.load_tables()?;
        let size = self.db_size()?;

        Ok(DbConnectionDetails {
            tables,
            version,
            db_size: size,
        })
    }

    fn load_tables(&mut self) -> Result<TableSummaries, AppError> {
        let query = format!(
            "
                SELECT table_name, table_rows, create_time, update_time
                FROM information_schema.tables
                WHERE table_schema = '{}'
                ORDER BY table_name;
            ",
            self.config.db_name.clone().unwrap()
        );

        let results = self.exec_query(&query)?;
        let tables: Vec<TableSummary> = results
            .iter()
            .map(|res| {
                let created = res.get("CREATE_TIME") as Option<Date>;
                let updated = res.get("CREATE_TIME") as Option<Date>;
                let name: String = res.get("TABLE_NAME").unwrap();

                let col_count = self.get_column_count(&name).unwrap();

                let table = Table {
                    name: name.clone(),
                    // conn_id: self.id.clone(),
                    config: None
                };

                self.tables = Vec::new();
                self.tables.push(Arc::new(Mutex::new(table)));

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

    fn table_exists(&self, name: &str) -> Result<bool, Self::Error> {
        let q = format!(
            "
                SELECT count(*) 
                FROM information_schema.tables
                WHERE table_schema = '{}' AND table_name = '{}'
            ",
            self.config.db_name.clone().unwrap(),
            name
        );

        let qr = self.exec_query(&q)?;
        let exists = qr.first().map_or(false, |r| r.get("count(*)").unwrap());

        Ok(exists)
    }

    fn get_table(&self, name: &str) -> Option<SharedTable> {
        for table in &self.tables {
            let t = table.lock().unwrap();

            if t.name == name {
                return Some(table.clone())
            }
        }

        None
    } 
    
}

#[cfg(test)]
mod test {
    use crate::base::{BasableConnection, AppError};

    use super::{Config, MysqlConn};

    fn create_db() -> Result<MysqlConn, AppError> {
        let db_name = "basable";
        let mut config = Config::default();

        config.db_name = Some(String::from(db_name));
        config.username = Some(String::from(db_name));
        config.password = Some(String::from("Basable@2024"));
        config.host = Some(String::from("localhost"));
        config.port = Some(3306);

        BasableConnection::new(config, "test_user")
    }

    #[test]
    fn test_table_count_summary() -> Result<(), AppError> {
        let mut db = create_db()?;
        db.load_tables()?;
        db.db_size()?;
        db.table_exists("swp")?;

        Ok(())
    }
}
