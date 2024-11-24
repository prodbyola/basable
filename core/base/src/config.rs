use axum::http::StatusCode;
use serde::Deserialize;
use urlencoding::encode;

use common::error::AppError;

#[derive(Deserialize, Clone, Debug)]
pub enum DatabaseType {
    Mysql,
    Postgres,
    Oracle,
    Mongo
}

impl TryFrom<&str> for DatabaseType {
    
    type Error = AppError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "postgres" => Ok(Self::Postgres),
            "oracle" => Ok(Self::Oracle),
            "mysql" => Ok(Self::Mysql),
            "mongo" => Ok(Self::Mongo),
            &_ => Err(AppError::HttpError(StatusCode::EXPECTATION_FAILED, "Invalid database source type".to_string())),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum Cloud { Firebase }

#[derive(Deserialize, Clone, Debug)]
pub enum SourceType {
    Database(DatabaseType), Cloud, File
}

impl SourceType {
    fn from_str(src_type: &str, src: &str) -> Result<SourceType, AppError> {
        match src_type {
            "database" => Ok(Self::Database(src.try_into()?)),
            "cloud" => Ok(Self::Cloud),
            "file" => Ok(Self::File),
            &_ => Err(AppError::HttpError(StatusCode::EXPECTATION_FAILED, "Invalid source type".to_string()))
        }
    }
}

/// Configuration options for a new `BasableConnection`.
#[derive(Deserialize, Clone, Debug)]
pub struct ConfigRaw {
    pub source_type: String,
    pub source: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub db_name: Option<String>,
}

impl Default for ConfigRaw {
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

impl ConfigRaw {
    pub fn build_url(&self) -> Result<String, AppError> {
        let src_type = SourceType::from_str(&self.source_type, &self.source)?;

        match src_type {
            SourceType::Database(_) => {
                let dbtype = &self.source;

                let username = self.username.clone().unwrap_or("root".to_string());
                let password = self.password.clone().unwrap_or_default();
                let host = self.host.clone().unwrap_or("localhost".to_string());
                let port = self.port.unwrap_or(3306);
                let db = self.db_name.clone().unwrap_or_default();

                let url = format!(
                    "{}://{}:{}@{}:{}/{}",
                    dbtype,
                    encode(&username),
                    encode(&password),
                    host,
                    port,
                    db
                );

                Ok(url)
            }

            _ => Err(AppError::not_implemented())
        }
        
    }

    pub fn get_source(&self) -> Result<SourceType, AppError> {
        SourceType::from_str(&self.source_type, &self.source)
    }
}
