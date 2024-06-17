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
    Arc<Mutex<dyn DB<Row = <MySqlDB as DB>::Row, Error = <MySqlDB as DB>::Error, ColumnValue = <MySqlDB as DB>::ColumnValue>>>;

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
    use crate::{
        base::AppError,
        tests::common::{create_test_instance, get_test_user_id},
    };

    #[test]
    fn test_instance_has_db() -> Result<(), AppError> {
        let bsbl = create_test_instance(true)?;
        let user_id = get_test_user_id();

        let user = bsbl.find_user(&user_id);
        assert!(user.is_some());

        let user = user.unwrap();
        let user = user.lock().unwrap();

        assert!(user.db().is_some());

        Ok(())
    }

    #[test]
    fn test_create_instance() {
        let bsbl = create_test_instance(true);
        assert!(bsbl.is_ok());
    }

    #[test]
    fn test_instance_has_user() -> Result<(), AppError> {
        let bsbl = create_test_instance(true)?;
        let user_id = get_test_user_id();

        let user = bsbl.find_user(&user_id);
        assert!(user.is_some());

        Ok(())
    }
}
