use std::str::FromStr;
use std::sync::Arc;

use uuid::Uuid;

use crate::imp::database::mysql::connector::MysqlConnector;
use crate::imp::database::mysql::db::MySqlDB;
use crate::User;

use super::connector::Connector;
use super::db::DB;
use super::table::TableConfigList;
use super::SharedDB;
use super::{
    config::{ConnectionConfig, Database, SourceType},
    user::{create_jwt, JwtSession},
    AppError,
};

#[derive(Default)]
pub(crate) struct Basable {
    pub connections: Vec<SharedDB>,
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
    pub(crate) fn create_guest_user(req_ip: &str) -> Result<JwtSession, AppError> {
        let user = User {
            id: req_ip.to_owned(),
            ..Default::default()
        };

        let session_id = create_jwt(user)?; // jwt encode the ip
        Ok(session_id)
    }

    /// Add connection.
    pub(crate) fn add_connection(&mut self, db: &SharedDB) {
        self.connections.push(db.clone());
    }

    pub fn get_connection(&self, id: &str, user_id: &str) -> Option<SharedDB> {
        let id = Uuid::from_str(id).unwrap();

        self.connections.iter()
            .find(|c| *c.id() == id && c.user_id() == user_id)
            .map(|c| c.clone())
    }
}
