use std::collections::HashMap;

use mysql::{Error as MySqlError, PooledConn};
use serde::Deserialize;
use urlencoding::encode;
use crate::base::{table::MysqlTable, MysqlConn};

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum RDMS {
    Mysql,
    Postgress,
    Oracle
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum NoSql { Sqlite }

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum SourceType {
    RDMS(RDMS), NoSql, File
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct Config {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub source_type: Option<SourceType>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: String::from("root"),
            password: Default::default(),
            host: String::from("localhost"),
            port: 3306,
            db_name: Default::default(),
            source_type: None,
        }
    }
}

impl Config {
    pub fn build_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            encode(self.username.as_str()),
            encode(self.password.as_str()),
            self.host,
            self.port,
            self.db_name
        )
    }
}

pub(crate) trait BasableConnection: Send + Sync {
    fn new(conn: Config) -> Self where Self: Sized;
    fn get_table(&mut self, table_name: &str) -> MysqlTable;
    fn table_names(&mut self) -> Result<Vec<String>, MySqlError>;
    fn first_table_name(&mut self) -> Result<Option<String>, MySqlError>;
}

pub(crate) struct User {
    pub is_logged: bool,
    pub connection: Option<Box<dyn BasableConnection>>,
}

impl User {
    /// TODO: Make this method fallible.
    pub(crate) fn create_connection(config: &Config) -> Option<Box<dyn BasableConnection>> {
        if let Some(src) = config.source_type {
            let db_src = match src {
                SourceType::RDMS(db) => {
                    match db {
                        RDMS::Mysql => MysqlConn::new(config.clone()),
                        _ => todo!(),
                    }
                },
                _ => todo!()
            };

            Some(Box::new(db_src))
        } else {
            None
        }
    }

    /// TODO: Make this method fallible.
    pub(crate) fn switch_connection(&mut self, config: &Config) {
        let connection = User::create_connection(&config).unwrap();
        self.connection = Some(connection);
    }

    pub(crate) fn validate(&self) -> bool {
        false
    }

    pub(crate) fn logout(&mut self){
        // TODO: Close connection
    }
}

#[derive(Default)]
pub(crate) struct Basable {
    pub users: HashMap<String, User>
}

impl Basable {
    fn add_user(&mut self, id: String, user: User){
        self.users.insert(id, user);
    }

    /// Creates a new guest user and add them to basable
    /// TODO: Make this method fallible.
    pub(crate) fn create_guest_user(&mut self, req_ip: &str, config: &Config) -> (String, &User) {
        let connection = User::create_connection(&config);
        
        let user = User {
            connection,
            is_logged: false,
        };

        let session_id = String::from(req_ip); // jwt encode the ip
        self.add_user(session_id, user);

        let user = self.find_user(&session_id).unwrap();
        (session_id, user)
    }


    pub(crate) fn find_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    pub(crate)fn log_user_out(&mut self, user_id: &str){
        if let Some(user) = self.find_user(user_id) {
            user.logout();
            self.users.remove(user_id);            
        }
    }
}