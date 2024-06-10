use std::sync::{Arc, Mutex};

use axum::http::StatusCode;

use crate::imp::database::mysql::{MySqlDB, MysqlConnector};
use crate::User;

use super::auth::SharedUser;
use super::{
    auth::{create_jwt, JwtSession},
    config::{Config, Database, SourceType},
    AppError, Connector, SharedConnection,
};

#[derive(Default)]
pub(crate) struct Basable {
    pub users: Vec<SharedUser>,
    // pub connections: Vec<SharedConnection>,
}

impl Basable {
    /// Creates a new thread-safe instance of `BasableConnection` as required by the `Config` parameter.
    pub(crate) fn create_connection(config: &Config) -> Result<Option<SharedConnection>, AppError> {
        let db = match config.source_type() {
            SourceType::Database(db) => match db {
                Database::Mysql => {
                    let conn = MysqlConnector::new(config.clone())?;
                    MySqlDB::new(conn)
                }
                _ => todo!(),
            },
            _ => todo!(),
        };

        Ok(Some(Arc::new(Mutex::new(db))))
    }

    /// Gets a user's `BasableConnection`.
    // pub(crate) fn get_connection(&self, user_id: &str) -> &Option<SharedConnection> {
    //     let user = &self.users.iter().find(|u| u.clone().lock().unwrap().id == user_id);
    //     if let Some(user) = user  {
    //         // let user = user.clone();
    //        return user.lock().unwrap().db()
    //     }

    //     &None
    // }

    // pub(crate) fn get_connection(&self, id: &Uuid) -> Option<&SharedConnection> {
    //     for conn in &self.connections {
    //         println!("Before lock {}", self.connections.len());
    //         let c = conn.lock().unwrap();
    //         println!("After lock {}", self.connections.len());
    //         if c.get_id() == *id {
    //             return Some(conn);
    //         }
    //     }

    //     None
    // }

    // pub(crate) fn conn_index(&self, user_id: &str) -> Option<usize> {
    //     self.connections
    //         .iter()
    //         .position(|c| c.lock().unwrap().get_user_id() == user_id)
    // }

    /// Creates a new guest user using the request `SocketAddr`
    pub(crate) fn create_guest_user(&mut self, req_ip: &str) -> Result<JwtSession, AppError> {
        let session_id = create_jwt(req_ip)?; // jwt encode the ip

        let mut user = User::default();
        user.id = req_ip.to_owned();

        self.add_user(user);

        Ok(session_id)
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(Arc::new(Mutex::new(user)));
    }

    /// Saves the `Config` to Basable's remote server in association with the user_id
    pub(crate) fn save_config(&mut self, config: &Config, user_id: &str) {
        let user = self.find_user(user_id);

        if let Some(user) = user {
            user.lock().unwrap().save_config(config);
        }
    }

    /// Get an active `User` with the `user_id` from Basable's active users.
    pub(crate) fn find_user(&self, user_id: &str) -> Option<SharedUser> {
        self.users
            .iter()
            .find(|u| u.lock().unwrap().id == user_id)
            .map(|u| u.clone())
    }

    // / Get a user's position index
    pub(crate) fn user_index(&self, user_id: &str) -> Option<usize> {
        self.users
            .iter()
            .position(|u| u.lock().unwrap().id == user_id)
    }

    /// Remove the user from Basable's active users.
    pub(crate) fn log_user_out(&mut self, user_id: &str) {
        if let Some(user) = self.find_user(user_id) {
            let i = self.user_index(user_id).unwrap();
            user.lock().unwrap().logout();
            self.users.remove(i);
        }
    }

    /// Attaches a DB to user.
    pub(crate) fn attach_db(
        &mut self,
        user_id: &str,
        db: SharedConnection,
    ) -> Result<(), AppError> {
        if let Some(user) = self.find_user(user_id) {
            user.lock().unwrap().attach_db(db);
            return Ok(());
        }

        Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to attach db to user. Looks like user does not exist.",
        ))
    }
}
