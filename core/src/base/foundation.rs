use std::cell::RefCell;
use std::str::FromStr;
use std::sync::Arc;

use uuid::Uuid;

use crate::imp::database::mysql::connector::MysqlConnector;
use crate::imp::database::mysql::db::MySqlDB;
use crate::User;

use super::connector::Connector;
use super::db::DB;
use super::table::TableConfigs;
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
}

impl Basable {
    /// Creates a new thread-safe instance of [`SharedDB`] as required by the [`Config`] parameter.
    ///
    /// The `auth_session` param should be set to `true` if current app [`User`] is logged.
    pub(crate) fn create_connection(
        config: &ConnectionConfig,
        user_id: String,
    ) -> Result<(SharedDB, TableConfigs), AppError> {
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

    /// Saves the `Config` to Basable's remote server in association with the user_id
    pub(crate) fn save_config(&mut self, config: &ConnectionConfig, user_id: &str) {
        let user = self.find_user(user_id);

        if let Some(user) = user {
            let user = user.borrow_mut();
            user.save_connection(config);
        }
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
        if let Some(user) = self.find_user(user_id) {
            let i = self.user_index(user_id).unwrap();
            user.take().logout();
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
}
