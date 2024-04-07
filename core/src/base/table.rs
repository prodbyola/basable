use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Utc};
use mysql::{
    consts::ColumnType,
    error::Error as MySqlError,
    prelude::{FromRow, Queryable},
    Params, PooledConn, Row, Statement, Value,
};

use super::{try_parse_date, RowCountOption};
use serde::Serialize;

pub struct ObaseTable {
    name: String,
    conn: PooledConn,
}

impl ObaseTable {
    pub fn new(conn: PooledConn, name: &str) -> Self {
        ObaseTable {
            name: String::from(name),
            conn,
        }
    }

    fn query(&mut self, query: String) -> Result<(Statement, Vec<Row>), MySqlError> {
        let conn = &mut self.conn;

        let stmt = conn.prep(query)?;
        let results = conn.exec(stmt.clone(), Params::Empty)?;
        Ok((stmt, results))
    }

    fn query_first<T: FromRow>(&mut self, query: String) -> Result<Option<T>, MySqlError> {
        let conn = &mut self.conn;
        conn.query_first(query)
    }

    pub fn row_count(&mut self, opt: Option<RowCountOption>) -> Result<u64, MySqlError> {
        let mut query = format!("SELECT COUNT(*) FROM {}", self.name);

        if let Some(opt) = opt {
            let col = opt.date_column;
            let fmt = "%Y-%m-%d";

            let date = match opt.date {
                Some(d) => {
                    let tm = try_parse_date(&d).unwrap();
                    tm.to_string()
                }
                None => {
                    let utc = Utc::now();
                    let local = utc.with_timezone(&Local);
                    local.format(fmt).to_string()
                }
            };

            println!("date: {}", date);

            query = format!(
                "SELECT COUNT(*) AS records_added_on_date FROM {} WHERE DATE({}) = {}",
                self.name, col, date
            );
        }

        if let Ok(Some(count)) = self.query_first::<u64>(query) {
            return Ok(count);
        }

        Ok(0)
    }

    /// Gets a string list of column names in the table
    pub fn show_columns(&mut self) -> Result<Vec<String>, MySqlError> {
        let query = format!(
            r#"SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = '{}'"#,
            self.name
        );

        let (_, res) = self.query(query)?;

        let mut cols = Vec::with_capacity(res.len());

        for row in res {
            let cn = "COLUMN_NAME";

            if let Some(value) = row.get(cn) {
                match value {
                    Value::Bytes(v) => {
                        let v = String::from_utf8(v).unwrap();
                        cols.push(v)
                    }
                    _ => println!(
                        "Unexpected column type {}, {}, {}",
                        file!(),
                        line!(),
                        column!()
                    ),
                }
            }
        }

        Ok(cols)
    }

    pub fn load_rows(&mut self) -> Result<ObaseRows, MySqlError> {
        let (stmt, rows) = self.query(format!(r#"SELECT * FROM {}"#, self.name))?;
        let cols = stmt.columns();

        let mut row_data = Vec::with_capacity(rows.len());

        for row in &rows {
            let mut row_cols = Vec::with_capacity(cols.len());

            for item in cols {
                let name = item.name_str().to_string();
                let value: Value = row.get(name.as_ref()).unwrap();
                let col_type = item.column_type();

                let col = ObaseColumn::new(name, value.into(), col_type.into());
                row_cols.push(col);
            }

            row_data.push(row_cols);
        }

        Ok(row_data)
    }
}

#[derive(Serialize, Debug)]
enum OBColumnValue {
    Bytes(Vec<u8>),
    NULL,
    Int(i64),
    UInt(u64),
    Float(f32),
    Double(f64),
    // year, month, day, hour, minutes, seconds, micro seconds
    Date(u16, u8, u8, u8, u8, u8, u32),
    // is negative, days, hours, minutes, seconds, micro seconds
    Time(bool, u32, u8, u8, u8, u32),
}

impl From<Value> for OBColumnValue {
    fn from(value: Value) -> OBColumnValue {
        match value {
            Value::Bytes(v) => OBColumnValue::Bytes(v),
            Value::NULL => OBColumnValue::NULL,
            Value::Int(v) => OBColumnValue::Int(v),
            Value::UInt(v) => OBColumnValue::UInt(v),
            Value::Float(v) => OBColumnValue::Float(v),
            Value::Double(v) => OBColumnValue::Double(v),
            Value::Date(yy, mm, dd, hh, min, sec, ms) => {
                OBColumnValue::Date(yy, mm, dd, hh, min, sec, ms)
            }
            Value::Time(n, dd, hh, min, sec, ms) => OBColumnValue::Time(n, dd, hh, min, sec, ms),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Debug)]
enum OBColumnType {
    DECIMAL,
    TINY,
    SHORT,
    LONG,
    FLOAT,
    DOUBLE,
    NULL,
    TIMESTAMP,
    LONGLONG,
    INT24,
    DATE,
    TIME,
    DATETIME,
    BLOB,
    VARCHAR,
    STRING,
    VAR_STRING,
    YEAR,
    NEWDATE,
    BIT,
    TIMESTAMP2,
    DATETIME2,
    TIME2,
    TYPED_ARRAY,
    UNKNOWN,
    JSON,
    NEWDECIMAL,
    ENUM,
    SET,
    TINY_BLOB,
    MEDIUM_BLOB,
    LONG_BLOB,
    GEOMETRY,
}

impl From<ColumnType> for OBColumnType {
    fn from(value: ColumnType) -> Self {
        match value {
            ColumnType::MYSQL_TYPE_DECIMAL => OBColumnType::DECIMAL,
            ColumnType::MYSQL_TYPE_TINY => OBColumnType::TINY,
            ColumnType::MYSQL_TYPE_SHORT => OBColumnType::SHORT,
            ColumnType::MYSQL_TYPE_LONG => OBColumnType::LONG,
            ColumnType::MYSQL_TYPE_FLOAT => OBColumnType::FLOAT,
            ColumnType::MYSQL_TYPE_DOUBLE => OBColumnType::DOUBLE,
            ColumnType::MYSQL_TYPE_NULL => OBColumnType::NULL,
            ColumnType::MYSQL_TYPE_TIMESTAMP => OBColumnType::TIMESTAMP,
            ColumnType::MYSQL_TYPE_LONGLONG => OBColumnType::LONGLONG,
            ColumnType::MYSQL_TYPE_INT24 => OBColumnType::INT24,
            ColumnType::MYSQL_TYPE_DATE => OBColumnType::DATE,
            ColumnType::MYSQL_TYPE_TIME => OBColumnType::TIME,
            ColumnType::MYSQL_TYPE_DATETIME => OBColumnType::DATETIME,
            ColumnType::MYSQL_TYPE_YEAR => OBColumnType::YEAR,
            ColumnType::MYSQL_TYPE_NEWDATE => OBColumnType::NEWDATE,
            ColumnType::MYSQL_TYPE_VARCHAR => OBColumnType::VARCHAR,
            ColumnType::MYSQL_TYPE_BIT => OBColumnType::BIT,
            ColumnType::MYSQL_TYPE_TIMESTAMP2 => OBColumnType::TIMESTAMP2,
            ColumnType::MYSQL_TYPE_DATETIME2 => OBColumnType::DATETIME2,
            ColumnType::MYSQL_TYPE_TIME2 => OBColumnType::TIME2,
            ColumnType::MYSQL_TYPE_TYPED_ARRAY => OBColumnType::TYPED_ARRAY,
            ColumnType::MYSQL_TYPE_UNKNOWN => OBColumnType::UNKNOWN,
            ColumnType::MYSQL_TYPE_JSON => OBColumnType::JSON,
            ColumnType::MYSQL_TYPE_NEWDECIMAL => OBColumnType::NEWDECIMAL,
            ColumnType::MYSQL_TYPE_ENUM => OBColumnType::ENUM,
            ColumnType::MYSQL_TYPE_SET => OBColumnType::SET,
            ColumnType::MYSQL_TYPE_TINY_BLOB => OBColumnType::TINY_BLOB,
            ColumnType::MYSQL_TYPE_MEDIUM_BLOB => OBColumnType::MEDIUM_BLOB,
            ColumnType::MYSQL_TYPE_LONG_BLOB => OBColumnType::LONG_BLOB,
            ColumnType::MYSQL_TYPE_BLOB => OBColumnType::BLOB,
            ColumnType::MYSQL_TYPE_VAR_STRING => OBColumnType::VAR_STRING,
            ColumnType::MYSQL_TYPE_STRING => OBColumnType::STRING,
            ColumnType::MYSQL_TYPE_GEOMETRY => OBColumnType::GEOMETRY,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ObaseColumn {
    name: String,
    value: OBColumnValue,
    col_type: OBColumnType,
}

impl ObaseColumn {
    fn new(name: String, value: OBColumnValue, col_type: OBColumnType) -> Self {
        Self {
            name,
            value,
            col_type,
        }
    }
}

pub type ObaseColumns = Vec<ObaseColumn>;
pub type ObaseRows = Vec<ObaseColumns>;
