use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use axum::http::StatusCode;

use crate::imp::database::mysql::connector::MysqlConnector;
use crate::imp::database::mysql::db::MySqlDB;
use crate::User;

use super::connector::Connector;
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
}

impl Basable {
    /// Creates a new thread-safe instance of [`SharedDB`] as required by the [`Config`] parameter.
    /// 
    /// The `auth_session` param should be set to `true` if current app [`User`] is logged.
    pub(crate) fn create_shared_db(config: &ConnectionConfig) -> Result<(SharedDB, TableConfigs), AppError> {
        let db = match config.source_type() {
            SourceType::Database(db) => match db {
                Database::Mysql => {
                    let conn = MysqlConnector::new(config.clone())?;
                    MySqlDB::new(Arc::new(conn))
                }
                _ => todo!(),
            },
            _ => todo!(),
        };

        let pointer: SharedDB = Arc::new(Mutex::new(db));
        let db = pointer.clone();
        let mut db = db.lock().unwrap();
        
        let conn = db.connector().clone();
        let table_configs = db.load_tables(conn)?;

        Ok((pointer, table_configs))
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
    pub(crate) fn attach_db(&mut self, user_id: &str, db: SharedDB) -> Result<(), AppError> {
        if let Some(user) = self.find_user(user_id) {
            user.borrow_mut().attach_db(db);
            return Ok(());
        }

        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to attach db to user. Looks like user does not exist.",
        ))
    }
}
