use serde::{Deserialize, Serialize};
use mysql::Value;

pub mod db;
pub mod connector;
pub mod table;
pub mod graphs;

/// Client side representation of a value of MySql column.
///
/// The `Value` is also used as a parameter to a prepared statement.
#[derive(Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum ColumnValue {
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

impl From<Value> for ColumnValue {
    fn from(value: Value) -> Self {
        match value {
            Value::NULL => ColumnValue::NULL,
            Value::Bytes(buf) => {
                let s = String::from_utf8(buf).unwrap();
                ColumnValue::Text(s)
            },
            Value::Int(v) => ColumnValue::Int(v),
            Value::UInt(v) => ColumnValue::UInt(v),
            Value::Float(v) => ColumnValue::Float(v),
            Value::Double(v) => ColumnValue::Double(v),
            Value::Date(y, m, d, h, min, sec, ms) => ColumnValue::Date(y, m, d, h, min, sec, ms),
            Value::Time(neg, d, h, min, sec, ms) => ColumnValue::Time(neg, d, h, min, sec, ms),
        }
    }
}