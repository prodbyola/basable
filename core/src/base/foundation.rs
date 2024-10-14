use std::str::FromStr;
use std::sync::Arc;

use axum::http::StatusCode;
use uuid::Uuid;

use crate::imp::database::mysql::connector::MysqlConnector;
use crate::imp::database::mysql::db::MySqlDB;
use crate::User;

use super::imp::connector::Connector;
use super::imp::db::DB;
use super::imp::SharedDB;
use super::{
    config::{ConfigRaw, DatabaseType, SourceType},
    user::{create_jwt, JwtSession},
    HttpError,
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
        config: &ConfigRaw,
        user_id: String,
    ) -> Result<SharedDB, HttpError> {

        let mut db = match config.get_source()? {
            SourceType::Database(db) => match db {
                DatabaseType::Mysql => {
                    let conn = MysqlConnector::new(config.clone())?;
                    let db = MySqlDB::new(Arc::new(conn), user_id);
                    Ok(db)
                }
                _ => Err(HttpError::not_implemented()),
            },
            _ => Err(HttpError::not_implemented()),
        }?;

        let conn = db.connector().clone();
        db.load_tables(conn)?;

        Ok(Arc::new(db))
    }

    /// Creates a new guest user using the request `SocketAddr`
    pub(crate) fn create_guest_user(req_ip: &str) -> Result<JwtSession, HttpError> {
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
