use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum RDMS {
    Mysql,
    Postgres,
    Oracle
}

impl From<&str> for RDMS {
    fn from(value: &str) -> Self {
        match value {
            "postgres" => Self::Postgres,
            "oracle" => Self::Oracle,
            &_ => Self::Mysql,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum NoSql { Sqlite }

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum SourceType {
    RDMS(RDMS), NoSql, File
}

impl SourceType {
    fn from_str(src_type: &str, src_val: &str) -> SourceType {
        match src_type {
            "rdms" => Self::RDMS(src_val.into()),
            "nosql" => Self::NoSql,
            &_ => Self::File
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct Config {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
    source_type: String,
    source: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: String::from("root"),
            password: Default::default(),
            host: String::from("localhost"),
            port: 3306,
            db_name: Default::default(),
            source_type: String::from("rdms"),
            source: String::from("mysql")
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

    pub fn source_type(&self) -> SourceType {
        SourceType::from_str(&self.source_type, &self.source)
    }
}
