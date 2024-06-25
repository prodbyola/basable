use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use uuid::Uuid;

use crate::imp::database::mysql::connector::MysqlConnector;
use crate::imp::database::mysql::db::MySqlDB;
use crate::User;

use super::connector::Connector;
use super::db::DB;
use super::table::{TableConfig, TableConfigList};
use super::SharedDB;
use super::{
    config::{ConnectionConfig, Database, SourceType},
    user::{create_jwt, JwtSession},
    AppError,
};

#[derive(Default)]
pub(crate) struct Basable {
    pub connections: RefCell<Vec<SharedDB>>,
    pub table_configs: HashMap<String, TableConfigList>,
}

impl Basable {
    /// Creates a new thread-safe instance of [`SharedDB`] as required by the [`Config`] parameter.
    ///
    /// The `auth_session` param should be set to `true` if current app [`User`] is logged.
    pub(crate) fn create_connection(
        config: &ConnectionConfig,
        user_id: String,
    ) -> Result<(SharedDB, Option<TableConfigList>), AppError> {
        let mut db = match config.source_type() {
            SourceType::Database(db) => match db {
                Database::Mysql => {
                    let conn = MysqlConnector::new(config.clone())?;
                    MySqlDB::new(Arc::new(conn), user_id)
                }
                _ => todo!(),
            },
            _ => todo!(),
        };

        let conn = db.connector().clone();
        let table_configs = db.load_tables(conn)?;

        Ok((Arc::new(db), table_configs))
    }

    /// Creates a new guest user using the request `SocketAddr`
    pub(crate) fn create_guest_user(&self, req_ip: &str) -> Result<JwtSession, AppError> {
        let user = User {
            id: req_ip.to_owned(),
            ..Default::default()
        };

        let session_id = create_jwt(user)?; // jwt encode the ip
        Ok(session_id)
    }

    /// Add connection.
    pub(crate) fn add_connection(&self, db: &SharedDB) {
        let get_conns = self.connections.borrow();
        let mut conns = get_conns.clone();

        // Drop get_conns immediately to avoid panick when replace it
        std::mem::drop(get_conns);

        conns.push(db.clone());
        self.connections.replace(conns.clone());
    }

    pub fn get_connection(&self, id: &str, user_id: &str) -> Option<SharedDB> {
        let id = Uuid::from_str(id).unwrap();

        let get_conns = self.connections.borrow();
        let conns = get_conns.clone();

        // Drop get_conns immediately to avoid panick when replace it
        std::mem::drop(get_conns);

        conns.iter()
            .find(|c| *c.id() == id && c.user_id() == user_id)
            .map(|c| c.clone())
    }

    pub fn add_configs(&mut self, conn_id: String, configs: TableConfigList) {
        self.table_configs.insert(conn_id, configs);
    }

    pub fn get_table_config(&self, conn_id: &str, table_name: &str) -> Option<TableConfig> {
        let mut config = None;
        if let Some(configs) = self.table_configs.get(conn_id) {
            config = configs
                .iter()
                .find(|c| c.borrow().table_id == table_name)
                .map(|c| c.borrow().clone())
        }

        config
    }

    pub fn save_table_config(&self, conn_id: &str, table_name: &str, config: TableConfig) {
        if let Some(configs) = self.table_configs.get(conn_id) {
            if let Some(c) = configs.iter().find(|c| c.borrow().table_id == table_name) {
                c.replace(config);
            }
        }
    }
}
