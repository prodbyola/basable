use std::str::FromStr;
use std::sync::Arc;

use axum::http::StatusCode;
use base::config::{ConfigRaw, DatabaseType, SourceType};
use base::connector::Connector;
use base::db::DB;
use base::mysql_plugin::connector::MysqlConnector;
use base::mysql_plugin::db::MySqlDB;
use base::SharedDB;
use common::error::AppError;
use uuid::Uuid;

use crate::user::{create_jwt, JwtSession, User};

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
    ) -> Result<SharedDB, AppError> {
        let mut db = match config.get_source()? {
            SourceType::Database(db) => match db {
                DatabaseType::Mysql => {
                    let conn = MysqlConnector::new(config.clone())?;
                    let db = MySqlDB::new(Arc::new(conn), user_id);
                    Ok(db)
                }
                _ => Err(AppError::not_implemented()),
            },
            _ => Err(AppError::not_implemented()),
        }?;

        let conn = db.connector().clone();
        db.load_tables(conn)?;

        Ok(Arc::new(db))
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

    pub fn get_connection(&self, id: &str, user_id: &str) -> Result<SharedDB, AppError> {

        match Uuid::from_str(id) {
            Ok(id) => {
                let conn = self
                    .connections
                    .iter()
                    .find(|c| *c.id() == id && c.user_id() == user_id)
                    .map(|c| c.clone());

                match conn {
                    Some(conn) => Ok(conn),
                    None => Err(AppError::HttpError(StatusCode::NOT_FOUND, "Connection instance not found".to_string()))
                }
            }
            Err(err) => Err(AppError::ServerError(err.to_string()))
        }
    }
}
