#[cfg(test)]
pub(crate) mod common {
    use dotenv::dotenv;
    use std::{
        env,
        sync::{Arc, Mutex},
    };

    use crate::base::{
        config::ConfigRaw, foundation::Basable, imp::SharedDB, AppError, AppState
    };

    /// Get `TEST_USER_ID` from env
    pub fn get_test_user_id() -> String {
        dotenv().ok();
        env::var("TEST_USER_ID").unwrap()
    }

    /// Get `TEST_DB_TABLE_NAME` from env
    pub fn get_test_db_table() -> String {
        dotenv().ok();
        env::var("TEST_DB_TABLE_NAME").unwrap()
    }

    /// Creates a test `Config`.
    pub fn create_test_config() -> ConfigRaw {
        dotenv().ok();

        let db_name = env::var("TEST_DB_NAME").unwrap();
        let db_username = env::var("TEST_DB_USERNAME").unwrap();
        let db_password = env::var("TEST_DB_PASSWORD").unwrap();
        let db_host = env::var("TEST_DB_HOST").unwrap();
        let db_port = env::var("TEST_DB_PORT").unwrap();
        let source = env::var("TEST_DB_SOURCE").unwrap();
        let source_type = env::var("TEST_DB_SOURCE_TYPE").unwrap();

        ConfigRaw {
            db_name: Some(db_name),
            username: Some(db_username),
            password: Some(db_password),
            host: Some(db_host),
            port: Some(db_port.parse().unwrap()),
            source,
            source_type,
        }
    }

    pub fn create_test_db() -> Result<SharedDB, AppError> {
        dotenv().ok();

        let user_id = get_test_user_id();
        let config = create_test_config();

        let conn = Basable::create_connection(&config, user_id)?;
        Ok(conn)
    }

    /// Creates a `Basable` instance for testing.
    ///
    /// Attaches a test `DB` instance if `attach_db` is `true`.
    pub fn create_test_instance(attach_db: bool) -> Result<Basable, AppError> {
        dotenv().ok();

        let mut bslb = Basable::default();

        if attach_db {
            // TODO: Save configs
            let db = create_test_db()?;
            bslb.add_connection(&db);
        }

        Ok(bslb)
    }

    /// Creates an `AppState` for testing.
    ///
    /// Attaches a test [DB](`crate::base::db::DB`) instance if `attach_db` is `true`.
    pub fn create_test_state(attach_db: bool) -> Result<AppState, AppError> {
        let instance = create_test_instance(attach_db)?;
        let state = AppState {
            instance: Arc::new(Mutex::new(instance)),
            ..Default::default()
        };

        Ok(state)
    }
}

#[cfg(test)]
pub(crate) mod extractors {
    use crate::{
        base::{user::User, AppError},
        http::middlewares::{AuthExtractor, DbExtractor, TableExtractor},
    };

    use super::common::{create_test_db, get_test_db_table, get_test_user_id};

    pub fn auth_extractor() -> AuthExtractor {
        let id = get_test_user_id();
        AuthExtractor(User {
            id,
            is_guest: false,
        })
    }

    pub fn db_extractor() -> Result<DbExtractor, AppError> {
        let db = create_test_db()?;
        Ok(DbExtractor(db))
    }

    pub fn table_extractor() -> Result<TableExtractor, AppError> {
        let table_name = get_test_db_table();
        let db = create_test_db()?;

        let table = db.get_table(&table_name).unwrap();

        Ok(TableExtractor(table.clone()))
    }
}
