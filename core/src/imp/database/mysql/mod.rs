use std::collections::HashMap;

use axum::http::StatusCode;
use mysql::Result;
use mysql::{prelude::Queryable, Opts, Params, Pool, Row};
use time::Date;

use crate::base::config::Config;
use crate::base::foundation::{BasableConnection, BasableTable};
use crate::base::AppError;

use super::{DBVersion, DbConnectionDetails, TableConfig, TableList};

/// MySQL implementation of `BasableConnection`
#[derive(Clone, Default)]
pub struct MysqlConn {
    /// Database connection pool
    pool: Option<Pool>,

    /// Connection options
    config: Config,

    /// A map of table configurations for the current connection. This map is
    /// saved for guest users/connections. Otherwise, configurations are saved on
    /// remote server.
    table_configs: Option<HashMap<String, TableConfig>>,
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

    fn new(config: Config) -> Result<Self, AppError> {
        let url = config.build_url();
        let opts = Opts::from_url(&url).unwrap();
        let pool = Pool::new(opts)?;

        Ok(MysqlConn {
            pool: Some(pool),
            config,
            table_configs: None,
        })
    }

    fn details(&self) -> Result<DbConnectionDetails, AppError> {
        let version = self.show_version()?;
        let tables = self.load_tables()?;
        let size = self.db_size()?;

        Ok(DbConnectionDetails {
            tables,
            version,
            db_size: size,
        })
    }

    fn load_tables(&self) -> Result<TableList, AppError> {
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
        let tables: Vec<BasableTable> = results
            .iter()
            .map(|res| {
                let created = res.get("CREATE_TIME") as Option<Date>;
                let updated = res.get("CREATE_TIME") as Option<Date>;
                let name: String = res.get("TABLE_NAME").unwrap();

                let col_count = self.get_column_count(&name).unwrap();

                BasableTable {
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

    fn save_table_config(
        &mut self,
        table_name: &str,
        table_config: TableConfig,
        save_local: bool,
    ) -> Result<(), Self::Error> {
        if save_local {
            let mut configs = self.table_configs.clone().unwrap_or_default();

            configs.insert(String::from(table_name), table_config);
            self.table_configs = Some(configs);
        } else {
            //TODO: Save remotely...
        }

        // self.table_configs = Some(configs);
        Ok(())
    }

    fn get_table_config(
        &mut self,
        table_name: &str,
        get_local: bool,
    ) -> Result<TableConfig, Self::Error> {

        let err_msg = "Unable to retrieve local config";
        let err= AppError::new(StatusCode::INTERNAL_SERVER_ERROR, &err_msg);
        
        if get_local {
            if let Some(tbs) = &self.table_configs {
                match tbs.get(table_name) {
                    Some(config) => {
                        let c = config.clone();
                        Ok(c)
                    },
                    None => Err(err)
                }
            } else {
                Err(err)
            }
        } else {
            Err(err)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::base::{foundation::BasableConnection, AppError};

    use super::{Config, MysqlConn};

    fn create_db() -> Result<MysqlConn, AppError> {
        let db_name = "basable";
        let mut config = Config::default();

        config.db_name = Some(String::from(db_name));
        config.username = Some(String::from(db_name));
        config.password = Some(String::from("Basable@2024"));
        config.host = Some(String::from("localhost"));
        config.port = Some(3306);

        BasableConnection::new(config)
    }

    #[test]
    fn test_table_count_summary() -> Result<(), AppError> {
        let db = create_db()?;
        db.load_tables()?;
        db.db_size()?;
        db.table_exists("swp")?;

        Ok(())
    }
}
