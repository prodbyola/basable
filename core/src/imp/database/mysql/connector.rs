use mysql::{prelude::Queryable, Opts, Params, Pool, Row};

use crate::base::{config::ConnectionConfig, connector::Connector, AppError};

/// MySQL implementation of `BasableConnection`
#[derive(Clone, Default)]
pub struct MysqlConnector {
    /// Database connection pool
    pub pool: Option<Pool>,

    /// Connection options
    pub config: ConnectionConfig,
    // table_configs: Option<HashMap<String, TableConfig>>,
}

impl MysqlConnector {
    fn pool(&self) -> Pool {
        self.pool.clone().unwrap()
    }
}

impl Connector for MysqlConnector {
    type Error = mysql::Error;
    type Row = Row;

    fn new(config: ConnectionConfig) -> Result<Self, AppError> {
        let url = config.build_url();
        let opts = Opts::from_url(&url).unwrap();
        let pool = Pool::new(opts)?;

        Ok(MysqlConnector {
            pool: Some(pool),
            config,
        })
    }

    fn exec_query(&self, query: &str) -> Result<Vec<Self::Row>, Self::Error> {
        let conn = &mut self.pool().get_conn()?;

        let stmt = conn.prep(query)?;
        conn.exec(stmt, Params::Empty)
    }

    fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}
