use mysql::{prelude::Queryable, Opts, Params, Pool, Row};
use serde::Deserialize;
use urlencoding::encode;

use self::table::ObaseTable;

mod table;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    username: String,
    password: String,
    host: String,
    port: u16,
    db_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: String::from("root"),
            password: Default::default(),
            host: String::from("localhost"),
            port: 3306,
            db_name: Default::default(),
        }
    }
}

impl Config {
    pub fn build_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            encode(self.username.as_str()),
            encode(self.password.as_str()),
            self.host,
            self.port,
            self.db_name
        )
    }
}

#[derive(Clone, Default)]
pub struct ObaseDB {
    pool: Option<Pool>,
    config: Config,
}

impl ObaseDB {
    pub fn new(config: Config) -> Self {
        let url = config.build_url();
        let opts = Opts::from_url(&url).unwrap();
        let pool = Pool::new(opts).unwrap();

        ObaseDB { pool: Some(pool), config }
    }

    fn pool(&mut self) -> Pool {
        self.pool.clone().unwrap()
    }

    /// Creates an instance of `ObasaTable` from its string name.
    pub fn table(&mut self, table_name: &str) -> ObaseTable {
        let conn = self.pool().get_conn().unwrap();
        ObaseTable::new(conn, table_name)
    }

    /// Runs a query to retrieve all tables (names) in the database as a list of string.
    pub fn table_names(&mut self) -> Vec<String> {
        let conn = &mut self.pool().get_conn().unwrap();

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

        res
    }
}

#[cfg(test)]
mod test {
    use super::{ObaseDB, Config};

    #[test]
    fn test_db(){
        let db_name = "basable";
        let mut config = Config::default();
        config.db_name = String::from(db_name);
        config.username = String::from(db_name);
        config.password = String::from("Basable@2024");

        let mut db = ObaseDB::new(config);

        let table_names = db.table_names();
        if !table_names.is_empty() {
            let mut table = db.table(table_names.first().unwrap());
            let cols = table.show_columns();

            assert!(cols.is_ok())
        }
    }

}
