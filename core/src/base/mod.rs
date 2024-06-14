use core::str;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use db::DB;

use crate::imp::database::mysql::db::MySqlDB;

pub(crate) mod column;
pub(crate) mod config;
pub(crate) mod connector;
pub(crate) mod db;
pub(crate) mod foundation;
pub(crate) mod table;
pub(crate) mod user;

pub(crate) type SharableDB =
    Arc<Mutex<dyn DB<Row = <MySqlDB as DB>::Row, Error = <MySqlDB as DB>::Error>>>;
/// A sharable connection that belongs to a specific user
// type SharedConnection = Arc<Mutex<impl DB>>;

#[derive(Debug)]
pub(crate) struct AppError(pub StatusCode, pub String);

impl AppError {
    pub(crate) fn new(code: StatusCode, msg: &str) -> Self {
        AppError(code, String::from(msg))
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

/// Implements conversion of `mysql::Error` to AppError. At the moment, all variations
/// of `mysql::Error` resolves to `StatusCode::INTERNAL_SERVER_ERROR`.
impl From<mysql::Error> for AppError {
    fn from(value: mysql::Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        (self.0, self.1).into_response()
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::{
        env,
        sync::{Arc, Mutex},
    };

    use crate::base::{foundation::Basable, AppError};

    use super::{config::Config, user::User};

    static TEST_USER_ID: &str = "test_user";

    fn create_instance() -> Result<Basable, AppError> {
        dotenv().ok();

        let db_name = env::var("TEST_DB_NAME").unwrap();
        let db_username = env::var("TEST_DB_USERNAME").unwrap();
        let db_password = env::var("TEST_DB_PASSWORD").unwrap();
        let db_host = env::var("TEST_DB_HOST").unwrap();
        let db_port = env::var("TEST_DB_PORT").unwrap();
        let source = env::var("TEST_DB_SOURCE").unwrap();
        let source_type = env::var("TEST_DB_SOURCE_TYPE").unwrap();

        let config = Config {
            db_name: Some(db_name),
            username: Some(db_username),
            password: Some(db_password),
            host: Some(db_host),
            port: Some(db_port.parse().unwrap()),
            source,
            source_type,
        };

        let user = User {
            id: TEST_USER_ID.to_owned(),
            ..User::default()
        };

        let mut bslb = Basable::default();
        bslb.add_user(Arc::new(Mutex::new(user)));

        let conn = Basable::create_connection(&config)?;
        bslb.attach_db(TEST_USER_ID, conn.unwrap())?;

        Ok(bslb)
    }

    #[test]
    fn test_create_db() -> Result<(), AppError> {
        let bsbl = create_instance()?;

        let user = bsbl.find_user(TEST_USER_ID);
        assert!(user.is_some());

        let user = user.unwrap();
        let user = user.lock().unwrap();

        assert!(user.db().is_some());

        Ok(())
    }

    #[test]
    fn test_create_instance() {
        let bsbl = create_instance();
        assert!(bsbl.is_ok());
    }

    #[test]
    fn test_has_user() -> Result<(), AppError> {
        let bsbl = create_instance()?;

        let user = bsbl.find_user(TEST_USER_ID);
        assert!(user.is_some());

        Ok(())
    }

    #[test]
    fn test_table_exist() -> Result<(), AppError> {
        let bsbl = create_instance()?;

        let user = bsbl.find_user(TEST_USER_ID);
        let user = user.unwrap();
        let user = user.lock().unwrap();

        let db = user.db();
        let db = db.unwrap();
        let mut db = db.lock().unwrap();

        db.load_tables()?;
        assert!(db.table_exists("swp")?);

        Ok(())
    }

    #[test]
    fn test_query_column() -> Result<(), AppError> {
        let bsbl = create_instance()?;

        let user = bsbl.find_user(TEST_USER_ID);
        let user = user.unwrap();
        let user = user.lock().unwrap();

        let db = user.db();
        let db = db.unwrap();
        let db = db.lock().unwrap();

        assert!(db.get_table("swp").is_some());

        if let Some(table) = db.get_table("swp") {
            let table = table.lock().unwrap();
            let cols = table.query_columns(db.connector());

            assert!(cols.is_ok());
        }

        Ok(())
    }
}
