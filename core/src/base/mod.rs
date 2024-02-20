use mysql::{prelude::Queryable, Opts, Params, Pool, Row};
use serde::Deserialize;
use urlencoding::encode;

use self::table::{ObaseRows, ObaseTable};

mod table;

#[derive(Deserialize, Clone)]
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
            host: String::from("root"),
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

    pub fn read_table_rows(&mut self, name: &str) -> ObaseRows {
        let conn = self.pool().get_conn().unwrap();
        let mut table = ObaseTable::new(conn, name);
        table.load_rows()
    }
}
