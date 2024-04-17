use std::collections::HashMap;

use chrono::NaiveDate;
use mysql::{prelude::Queryable, error::Error as MySqlError, Opts, Params, Pool, Row};

use crate::base::config::Config;
use crate::base::foundation::{BasableConnection, ConnectionDetails};
use crate::base::{AppError, ConnectionStatus};

use self::table::MysqlTable;

pub mod table;

pub enum CountDateSelection {
    Day,
    Month,
    Year
}

pub struct RowCountOption {
    pub date: Option<String>,
    pub date_column: String,
    pub date_selection: CountDateSelection
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

    fn get_status(&self) -> Result<ConnectionStatus, MySqlError> {
        let conn = &mut self.pool().get_conn()?;

        let stmt = conn.prep("SHOW STATUS")?;
        let status: Vec<Row> = conn.exec(stmt, Params::Empty)?;
        let mut data = HashMap::new();

        for s in status {
            let name: String = s.get("Variable_name").unwrap();
            let value: String = s.get("Value").unwrap();
            data.insert(name, value);
        }

        Ok(data)
    }
}

impl BasableConnection for MysqlConn {
    type Error = AppError;

    fn new(config: Config) -> Result<Self, AppError> {
        let url = config.build_url();
        let opts = Opts::from_url(&url).unwrap();
        let pool = Pool::new(opts)?;

        Ok(MysqlConn { pool: Some(pool), config })
    }

    fn get_details(&self) -> Result<ConnectionDetails, AppError> {

        let status = self.get_status()?;
        Ok(ConnectionDetails { status })
    }

    /// Creates an instance of `BasableTable` from its string name.
    fn get_table(&mut self, table_name: &str) -> MysqlTable {
        let conn = self.pool().get_conn().unwrap();
        MysqlTable::new(conn, table_name)
    }

    /// Runs a query to retrieve all tables (names) in the database as a list of string.
    fn table_names(&self) -> Result<Vec<String>, AppError> {
        let conn = &mut self.pool().get_conn()?;

        let query = format!(
            r#"SELECT *
            FROM information_schema.tables
            WHERE table_schema = '{}'"#,
            &self.config.db_name
        );

        let stmt = conn.prep(query).unwrap();
        let tables: Vec<Row> = conn.exec(stmt, Params::Empty).unwrap();

        let mut res = Vec::with_capacity(tables.len());

        if !tables.is_empty() {
            for item in tables {
                let row_name: String = item.get("TABLE_NAME").unwrap();
                res.push(row_name);
            }
        }

        Ok(res)
    }

    fn first_table_name(&mut self) -> Result<Option<String>, AppError> {
        let tbs = self.table_names()?;
        let mut f = None;

        if !tbs.is_empty() {
            f = Some(tbs[0].to_owned());
        } 

        Ok(f)

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
    use crate::{base::{foundation::BasableConnection, AppError}, imp::rdms::mysql::CountDateSelection};

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
    fn test_show_columns() -> Result<(), AppError>{
        let mut db = create_db()?;

        if let Some(tb) = db.first_table_name().unwrap() {
            let mut table = db.get_table(&tb);
            let cols = table.show_columns();

            assert!(cols.is_ok())
        }

        Ok(())
    }

    #[test]
    fn test_row_count() -> Result<(), AppError>{
        let mut db = create_db()?;

        let mut table = db.get_table("swp");
        table.row_count(None).unwrap();

        let opt = RowCountOption{
            date: Some(String::from("1/1/1981")),
            date_column: String::from("date"),
            date_selection: CountDateSelection::Day
        };

        let today = table.row_count(Some(opt)).unwrap();

        println!("today {}", today);

        Ok(())
    }

}
