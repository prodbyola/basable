use std::sync::{Arc, Mutex};

use crate::imp::database::mysql::MysqlConn;
use crate::User;

use super::{
    auth::{create_jwt, JwtSession},
    config::{Config, Database, SourceType},
    AppError, BasableConnection, SharedConnection,
};

#[derive(Default)]
pub(crate) struct Basable {
    pub users: Vec<User>,
    pub connections: Vec<SharedConnection>,
}

impl Basable {
    /// Creates a new thread-safe instance of `BasableConnection` as required by the `Config` parameter.
    pub(crate) fn create_connection(
        config: &Config,
        user_id: &str,
    ) -> Result<Option<SharedConnection>, AppError> {
        let conn = match config.source_type() {
            SourceType::Database(db) => match db {
                Database::Mysql => MysqlConn::new(config.clone(), user_id)?,
                _ => todo!(),
            },
            _ => todo!(),
        };

        Ok(Some(Arc::new(Mutex::new(conn))))
    }

    /// Gets a user's active `BasableConnection`.
    pub(crate) fn get_connection(&self, user_id: &str) -> Option<&SharedConnection> {
        for conn in &self.connections {
            let c = conn.lock().unwrap();
            if c.get_user_id() == user_id {
                return Some(conn);
            }
        }

        None
    }

    pub(crate) fn conn_index(&self, user_id: &str) -> Option<usize> {
        self.connections.iter().position(|c| c.lock().unwrap().get_user_id() == user_id)
    }

    /// Creates a new guest user using the request `SocketAddr`
    pub(crate) fn create_guest_user(&mut self, req_ip: &str) -> Result<JwtSession, AppError> {
        let session_id = create_jwt(req_ip)?; // jwt encode the ip

        let user = User {
            id: req_ip.to_owned(),
            is_logged: false,
        };

        self.add_user(user.clone());

        Ok(session_id)
    }

    /// Saves the `Config` to Basable's remote server in association with the user_id
    pub(crate) fn save_config(&mut self, config: &Config, user_id: &str) {
        let user = self
            .find_user(user_id)
            .expect("Unable to find user with id");
        user.save_config(config);
    }

    /// Get an active `User` with the `user_id` from Basable's active users.
    pub(crate) fn find_user(&self, user_id: &str) -> Option<&User> {
        for u in &self.users {
            if u.id == user_id {
                return Some(u);
            }
        }

        None
    }

    /// Get a user's position index
    pub(crate) fn user_index(&self, user_id: &str) -> Option<usize> {
        self.users.iter().position(|u| u.id == user_id)
    }

    /// Remove the user from Basable's active users.
    pub(crate) fn log_user_out(&mut self, user_id: &str) {
        if let Some(user) = self.find_user(user_id) {
            let i = self.user_index(user_id).unwrap();
            user.logout();
            self.users.remove(i);
        }
    }

    /// Adds a user to Basable's active user.
    fn add_user(&mut self, user: User) {
        // let id = user.id.clone();
        self.users.push(user);
    }

    /// Remove any existing user's `BasableConnection` and add this.
    pub(crate) fn add_connection(&mut self, user_id: &str, conn: SharedConnection) {
        if let Some(index) = self.conn_index(user_id) {
            self.connections.remove(index);
        }

        self.connections.push(conn);
    }
}
