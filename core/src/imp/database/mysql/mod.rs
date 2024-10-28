use serde::{Deserialize, Serialize};
use mysql::Value;

pub(crate) mod db;
pub(crate) mod connector;
pub(crate) mod table;
pub(crate) mod graphs;

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