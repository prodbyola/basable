use std::env;

use crate::base::config::Config;

pub(crate) fn create_test_config() -> Config {
    let db_name = env::var("TEST_DB_NAME").unwrap();
        let db_username = env::var("TEST_DB_USERNAME").unwrap();
        let db_password = env::var("TEST_DB_PASSWORD").unwrap();
        let db_host = env::var("TEST_DB_HOST").unwrap();
        let db_port = env::var("TEST_DB_PORT").unwrap();
        let source = env::var("TEST_DB_SOURCE").unwrap();
        let source_type = env::var("TEST_DB_SOURCE_TYPE").unwrap();

        Config {
            db_name: Some(db_name),
            username: Some(db_username),
            password: Some(db_password),
            host: Some(db_host),
            port: Some(db_port.parse().unwrap()),
            source,
            source_type,
        }
}