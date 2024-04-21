use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum Database {
    Mysql,
    Postgres,
    Oracle
}

impl Database {
    fn to_str(self) -> &'static str {
        match self {
            Database::Mysql => "mysql",
            Database::Postgres => "postgres",
            Database::Oracle => "oracle",
        }
    }
}

impl From<&str> for Database {
    fn from(value: &str) -> Self {
        match value {
            "postgres" => Self::Postgres,
            "oracle" => Self::Oracle,
            "mysql" => Self::Mysql,
            &_ => Self::Mysql,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum Cloud { Firebase }

#[derive(Deserialize, Clone, Debug)]
pub(crate) enum SourceType {
    Database(Database), Cloud, File
}

impl SourceType {
    fn from_str(src_type: &str, src_val: &str) -> SourceType {
        match src_type {
            "database" => Self::Database(src_val.into()),
            "cloud" => Self::Cloud,
            "file" => Self::File,
            &_ => Self::File
        }
    }
}

/// Configuration options for a new `BasableConnection`.
#[derive(Deserialize, Clone, Debug)]
pub(crate) struct Config {
    source_type: String,
    source: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub db_name: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
            host: None,
            port: None,
            db_name: None,
            source_type: String::from("database"),
            source: String::from("mysql")
        }
    }
}

impl Config {
    pub fn build_url(&self) -> String {
        let src_type = SourceType::from_str(&self.source_type, &self.source);

        match src_type {
            SourceType::Database(src) => {
                let src = src.to_str();

                let username = self.username.clone().unwrap_or("root".to_string());
                let password = self.password.clone().unwrap_or_default();
                let host = self.host.clone().unwrap_or("localhost".to_string());
                let port = self.port.unwrap_or(3306);
                let db = self.db_name.clone().unwrap_or_default();

                format!(
                    "{}://{}:{}@{}:{}/{}",
                    src,
                    encode(&username),
                    encode(&password),
                    host,
                    port,
                    db
                )
            }

            _ => String::new()
        }
        
    }

    pub fn source_type(&self) -> SourceType {
        SourceType::from_str(&self.source_type, &self.source)
    }
}
