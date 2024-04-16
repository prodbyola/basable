use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::base::{table::MysqlTable, MysqlConn};
use mysql::Error as MySqlError;

use self::{
    auth::User,
    config::{Config, SourceType, RDMS},
};

pub(crate) mod auth;
pub(crate) mod config;

type SharedConnection = Arc<Mutex<dyn BasableConnection>>;

pub(crate) trait BasableConnection: Send + Sync {
    fn new(conn: Config) -> Self
    where
        Self: Sized;
    fn get_table(&mut self, table_name: &str) -> MysqlTable;
    fn table_names(&mut self) -> Result<Vec<String>, MySqlError>;
    fn first_table_name(&mut self) -> Result<Option<String>, MySqlError>;
}

#[derive(Default)]
pub(crate) struct Basable {
    pub users: HashMap<String, User>,
    pub connections: HashMap<String, SharedConnection>,
}

impl Basable {
    /// TODO: Make this method fallible.
    pub(crate) fn create_connection(config: &Config) -> Option<SharedConnection> {
        let db_src = match config.source_type() {
            SourceType::RDMS(db) => match db {
                RDMS::Mysql => MysqlConn::new(config.clone()),
                _ => todo!(),
            },
            _ => todo!(),
        };

        Some(Arc::new(Mutex::new(db_src)))
    }

    pub(crate) fn get_connection(&self, user_id: &str) -> Option<&SharedConnection> {
        self.connections.get(user_id)
    }

    /// Creates a new guest user, create a new connection for the user using the `Config`
    /// and add the user to Basable. It returns new session-id for user
    ///
    /// TODO: Make this method fallible.
    pub(crate) fn create_guest_user(&mut self, req_ip: String, config: &Config) -> String {
        let session_id = String::from(req_ip); // jwt encode the ip

        let user = User {
            id: session_id,
            is_logged: false,
        };

        self.add_user(user.clone());

        if let Some(conn) = Basable::create_connection(&config) {
            self.add_connection(user.id.clone(), conn);
        }

        user.id
    }

    pub(crate) fn save_new_config(&mut self, config: &Config, user_id: &str) {
        let user = self
            .find_user(user_id)
            .expect("Unable to find user with id");
        user.save_new_config(config);
    }

    pub(crate) fn find_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    pub(crate) fn log_user_out(&mut self, user_id: &str) {
        if let Some(user) = self.find_user(user_id) {
            user.logout();
            self.users.remove(user_id);
        }
    }

    fn add_user(&mut self, user: User) {
        let id = user.id.clone();
        self.users.insert(id, user);
    }

    fn add_connection(&mut self, user_id: String, conn: SharedConnection) {
        // TODO: Find and close existing connection before insert a new one.
        self.connections.insert(user_id, conn);
    }
}
