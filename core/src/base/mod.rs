use chrono::NaiveDate;
use mysql::{prelude::Queryable, error::Error as MySqlError, Opts, Params, Pool, Row};

use crate::types::{BasableConnection, Config};

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
    fn pool(&mut self) -> Pool {
        self.pool.clone().unwrap()
    }
}

impl BasableConnection for MysqlConn {
    fn new(config: Config) -> Self {
        let url = config.build_url();
        let opts = Opts::from_url(&url).unwrap();
        let pool = Pool::new(opts).unwrap();

        MysqlConn { pool: Some(pool), config }
    }

    /// Creates an instance of `BasableTable` from its string name.
    fn get_table(&mut self, table_name: &str) -> MysqlTable {
        let conn = self.pool().get_conn().unwrap();
        MysqlTable::new(conn, table_name)
    }

    /// Runs a query to retrieve all tables (names) in the database as a list of string.
    fn table_names(&mut self) -> Result<Vec<String>, MySqlError> {
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

    fn first_table_name(&mut self) -> Result<Option<String>, MySqlError> {
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
    use crate::types::BasableConnection;

    use super::{Config, MysqlConn, RowCountOption};

    fn create_db() -> MysqlConn {
        let db_name = "basable";
        let mut config = Config::default();
        config.db_name = String::from(db_name);
        config.username = String::from(db_name);
        config.password = String::from("Basable@2024");

        BasableConnection::new(config)
    }

    #[test]
    fn test_show_columns(){
        let mut db = create_db();

        if let Some(tb) = db.first_table_name().unwrap() {
            let mut table = db.get_table(&tb);
            let cols = table.show_columns();

            assert!(cols.is_ok())
        }
    }

    #[test]
    fn test_row_count(){
        let mut db = create_db();

        let mut table = db.get_table("swp");
        table.row_count(None).unwrap();

        let opt = RowCountOption{
            date: Some(String::from("1/1/1981")),
            date_column: String::from("date"),
            date_selection: crate::base::CountDateSelection::Day
        };

        let today = table.row_count(Some(opt)).unwrap();

        println!("today {}", today);
    }

}
