use common::error::AppError;
use mysql::{prelude::Queryable, Opts, Params, Pool, Row};

use crate::{config::ConfigRaw, connector::Connector};

/// MySQL implementation of `BasableConnection`
#[derive(Clone, Default)]
pub struct MysqlConnector {
    /// Database connection pool
    pub pool: Option<Pool>,

    /// Connection options
    pub config: ConfigRaw,
    // table_configs: Option<HashMap<String, TableConfig>>,
}

impl MysqlConnector {
    fn pool(&self) -> Pool {
        self.pool.clone().unwrap()
    }
}

impl Connector for MysqlConnector {
    type Row = Row;

    fn new(config: ConfigRaw) -> Result<Self, AppError> {
        let url = config.build_url()?;
        let opts = Opts::from_url(&url).map_err(|err| AppError::ServerError(err.to_string()))?;

        Pool::new(opts)
            .map(|pool| MysqlConnector {
                pool: Some(pool),
                config,
            })
            .map_err(|err| AppError::ServerError(err.to_string()))
    }

    fn exec_query(&self, query: &str) -> Result<Vec<Self::Row>, AppError> {
        let conn = &mut self.pool().get_conn()?;

        let stmt = conn.prep(query)?;
        let rows = conn.exec(stmt, Params::Empty)?;

        Ok(rows)
    }

    fn config(&self) -> &ConfigRaw {
        &self.config
    }
}
