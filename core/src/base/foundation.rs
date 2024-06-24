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

pub(crate) type SharableUser = RefCell<User>;

#[derive(Default)]
pub(crate) struct Basable {
    pub users: Vec<SharableUser>,
    pub connections: Vec<SharedDB>,
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
    pub(crate) fn create_guest_user(&mut self, req_ip: &str) -> Result<JwtSession, AppError> {
        let session_id = create_jwt(req_ip)?; // jwt encode the ip

        let user = User {
            id: req_ip.to_owned(),
            ..Default::default()
        };

        self.add_user(RefCell::new(user));

        Ok(session_id)
    }

    pub fn add_user(&mut self, user: SharableUser) {
        self.users.push(user);
    }

    /// Get an active `User` with the `user_id` from Basable's active users.
    pub(crate) fn find_user(&self, user_id: &str) -> Option<&SharableUser> {
        self.users.iter().find(|u| u.borrow().id == user_id)
        // .map(|u| u.clone());
    }

    // / Get a user's position index
    pub(crate) fn user_index(&self, user_id: &str) -> Option<usize> {
        self.users.iter().position(|u| u.borrow().id == user_id)
    }

    /// Remove the user from Basable's active users.
    pub(crate) fn log_user_out(&mut self, user_id: &str) {
        if let Some(_) = self.find_user(user_id) {
            let i = self.user_index(user_id).unwrap();
            self.users.remove(i);
        }
    }

    /// Attaches a DB to user.
    pub(crate) fn add_connection(&mut self, db: &SharedDB) {
        // if let Some(_) = self.get_connection(db.user_id()) {
        //     let i = self
        //         .connections
        //         .iter()
        //         .position(|c| c.user_id() == db.user_id())
        //         .unwrap();
        //     self.connections.remove(i);
        // }

        self.connections.push(db.clone());
    }

    pub fn get_connection(&self, id: &str, user_id: &str) -> Option<SharedDB> {
        let id = Uuid::from_str(id).unwrap();

        self.connections
            .iter()
            .find(|c| *c.id() == id && c.user_id() == user_id)
            .map(|c| c.clone())
    }

    pub fn add_configs(&mut self, conn_id: String, configs: TableConfigList){
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
