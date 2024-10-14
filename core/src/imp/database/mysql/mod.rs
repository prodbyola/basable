use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::base::HttpError;
use mysql::Value;

pub(crate) mod db;
pub(crate) mod connector;
pub(crate) mod table;
pub(crate) mod graphs;

/// Implements conversion of `mysql::Error` to AppError. At the moment, all variations
/// of `mysql::Error` resolves to `StatusCode::INTERNAL_SERVER_ERROR`.
impl From<mysql::Error> for HttpError {
    fn from(value: mysql::Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

/// Client side representation of a value of MySql column.
///
/// The `Value` is also used as a parameter to a prepared statement.
#[derive(Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub(crate) enum MySqlValue {
    NULL,
    Text(String),
    Int(i64),
    UInt(u64),
    Float(f32),
    Double(f64),
    /// year, month, day, hour, minutes, seconds, micro seconds
    Date(u16, u8, u8, u8, u8, u8, u32),
    /// is negative, days, hours, minutes, seconds, micro seconds
    Time(bool, u32, u8, u8, u8, u32),
}

impl From<Value> for MySqlValue {
    fn from(value: Value) -> Self {
        match value {
            Value::NULL => MySqlValue::NULL,
            Value::Bytes(buf) => {
                let s = String::from_utf8(buf).unwrap();
                MySqlValue::Text(s)
            },
            Value::Int(v) => MySqlValue::Int(v),
            Value::UInt(v) => MySqlValue::UInt(v),
            Value::Float(v) => MySqlValue::Float(v),
            Value::Double(v) => MySqlValue::Double(v),
            Value::Date(y, m, d, h, min, sec, ms) => MySqlValue::Date(y, m, d, h, min, sec, ms),
            Value::Time(neg, d, h, min, sec, ms) => MySqlValue::Time(neg, d, h, min, sec, ms),
        }
    }
}