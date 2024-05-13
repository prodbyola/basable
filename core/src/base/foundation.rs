use std::sync::{Arc, Mutex};

use uuid::Uuid;

use super::table::{TableConfig, TableList};
use crate::imp::database::{mysql::MysqlConn, DbConnectionDetails};
use crate::User;

use super::{
    auth::{create_jwt, JwtSession},
    config::{Config, Database, SourceType},
    AppError, SharedConnection,
};

/// Basable base trait that must be implemented by every instance of connection in Basable.
///
/// Check `imp` module for different implementations of this trait.
pub(crate) trait BasableConnection: Send + Sync {
    type Error;
    /// A new instance of BasableConnection
    fn new(conn: Config, user_id: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn get_id(&self) -> Uuid;

    fn get_user_id(&self) -> &str;

    /// Details about the connection
    fn details(&self) -> Result<DbConnectionDetails, Self::Error>;

    /// Load table summaries
    fn load_tables(&self) -> Result<TableList, Self::Error>;

    /// Check if a table with the given name exists in the database connection.
    fn table_exists(&self, name: &str) -> Result<bool, Self::Error>;

    /// Saves a table configuration. If `save_local` is true, it saves in memore using
    /// `BasableConnection` instance. Otherwise, it saves to remote server.
    fn save_table_config(
        &mut self,
        table_name: &str,
        table_config: TableConfig,
        save_local: bool,
    ) -> Result<(), Self::Error>;

    fn get_table_config(
        &mut self,
        table_name: &str,
        get_local: bool,
    ) -> Result<TableConfig, Self::Error>;
}

#[derive(Default)]
pub(crate) struct Basable {
    pub users: Vec<User>,
    pub connections: Vec<SharedConnection>,
}

impl Basable {
    /// Creates a new thread-safe instance of `BasableConnection` as required by the `Config` parameter.
    pub(crate) fn create_connection(config: &Config, user_id: &str) -> Result<Option<SharedConnection>, AppError> {
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
                return Some(conn)
            }
        }
        
        None
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

    /// Adds a `BasableConnection` to active connections.
    pub(crate) fn add_connection(&mut self, conn: SharedConnection) {
        // TODO: Find and close existing connection before inserting a new one.
        self.connections.push(conn);
    }
}
