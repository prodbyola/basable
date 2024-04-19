use std::collections::HashMap;

use chrono::NaiveDate;
use mysql::{Result, Value};
use mysql::{prelude::Queryable, Opts, Params, Pool, Row};
use time::Date;

use crate::base::config::Config;
use crate::base::foundation::{BasableConnection, ConnectionDetails, TableSummary};
use crate::base::{AppError, ConnectionStatus, TableSummaries};

use self::table::MysqlTable;

pub mod table;

pub enum CountDateSelection {
    Day,
    Month,
    Year,
}

pub struct RowCountOption {
    pub date: Option<String>,
    pub date_column: String,
    pub date_selection: CountDateSelection,
}

/// An instance of Basable Database
#[derive(Clone, Default)]
pub struct MysqlConn {
    pool: Option<Pool>,
    config: Config,
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

    fn show_status(&self) -> Result<ConnectionStatus, AppError> {
        let status = self.exec_query("SHOW STATUS")?;
        let mut data = HashMap::new();

        for s in status {
            let name: String = s.get("Variable_name").unwrap();
            let value: String = s.get("Value").unwrap();
            data.insert(name, value);
        }

        Ok(data)
    }

    fn show_variables(&self) -> Result<ConnectionStatus, AppError> {
        let vars = self.exec_query("SHOW VARIABLES")?;
        let mut data = HashMap::new();

        for v in vars {
            let name: String = v.get("Variable_name").unwrap();
            let value: String = v.get("Value").unwrap();
            data.insert(name, value);
        }

        Ok(data)
    }

    fn get_table_summary(&self) -> Result<TableSummaries, AppError> {
        let query = format!("
            SELECT table_name, table_rows, create_time, update_time
            FROM information_schema.tables
            WHERE table_schema = '{}'
            ORDER BY table_name;
        ", self.config.db_name
    );

        let results = self.exec_query(&query)?;
        let tables: Vec<TableSummary> = results.iter().map(|res|{
            let created = res.get("CREATE_TIME") as Option<Date>;
            let updated = res.get("CREATE_TIME") as Option<Date>;
            
            TableSummary {
                name: res.get("TABLE_NAME").unwrap(),
                row_count: res.get("TABLE_ROWS").unwrap(),
                created: created.map_or(None, |d| Some(d.to_string())),
                updated: updated.map_or(None, |d| Some(d.to_string()))
            }
        }).collect();

        Ok(tables)
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
        })
    }

    fn get_details(&self) -> Result<ConnectionDetails, AppError> {
        let status = self.show_status()?;
        let variables = self.show_variables()?;
        let tables = self.get_table_summary()?;
        Ok(ConnectionDetails { tables, status, variables })
    }

    /// Creates an instance of `BasableTable` from its string name.
    fn get_table(&mut self, table_name: &str) -> MysqlTable {
        let conn = self.pool().get_conn().unwrap();
        MysqlTable::new(conn, table_name)
    }
}

pub fn try_parse_date(date_str: &str) -> Option<NaiveDate> {
    // List of potential date formats to try
    let date_formats = [
        "%Y-%m-%d", // Format: YYYY-MM-DD
        "%m/%d/%Y", // Format: MM/DD/YYYY
        "%d/%m/%Y", // Format: DD/MM/YYYY
                    // Add more formats as needed
    ];

    // Try parsing the date using each format
    for format in date_formats {
        if let Ok(parsed_date) = NaiveDate::parse_from_str(date_str, format) {
            return Some(parsed_date);
        }
    }

    // If none of the formats succeeded, return None
    None
}

#[cfg(test)]
mod test {
    use crate::{
        base::{foundation::BasableConnection, AppError},
        imp::rdms::mysql::CountDateSelection,
    };

    use super::{Config, MysqlConn, RowCountOption};

    fn create_db() -> Result<MysqlConn, AppError> {
        let db_name = "basable";
        let mut config = Config::default();

        config.db_name = String::from(db_name);
        config.username = String::from(db_name);
        config.password = String::from("Basable@2024");

        BasableConnection::new(config)
    }

    #[test]
    fn test_table_count_summary() -> Result<(), AppError> {
        let db = create_db()?;
        db.get_table_summary()?;

        Ok(())
    }

    #[test]
    fn test_row_count() -> Result<(), AppError> {
        let mut db = create_db()?;

        let mut table = db.get_table("swp");
        table.row_count(None).unwrap();

        let opt = RowCountOption {
            date: Some(String::from("1/1/1981")),
            date_column: String::from("date"),
            date_selection: CountDateSelection::Day,
        };

        let today = table.row_count(Some(opt));
        assert!(today.is_ok());

        Ok(())
    }
}
