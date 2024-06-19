#[cfg(test)]
pub(crate) mod common {
    use dotenv::dotenv;
    use std::{
        cell::RefCell, env, sync::{Arc, Mutex}
    };

    use crate::{
        base::{config::Config, foundation::Basable, user::User, AppError},
        http::{app::AppState, middlewares::AuthExtractor},
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

    pub fn create_test_auth_extractor() -> AuthExtractor {
        let user_id = get_test_user_id();
        AuthExtractor(Some(user_id))
    }

    /// Creates a test `Config`.
    pub fn create_test_config() -> Config {
        dotenv().ok();

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

    /// Creates a `Basable` instance for testing. 
    /// 
    /// Attaches a test `DB` instance if `attach_db` is `true`.
    pub fn create_test_instance(attach_db: bool) -> Result<Basable, AppError> {
        dotenv().ok();

        let user_id = get_test_user_id();

        let config = create_test_config();
        let user = User {
            id: user_id.clone(),
            ..User::default()
        };

        let mut bslb = Basable::default();
        bslb.add_user(RefCell::new(user));

        if attach_db {
            let conn = Basable::create_connection(&config)?;
            bslb.attach_db(&user_id, conn)?;
        }

        Ok(bslb)
    }

    /// Creates an `AppState` for testing. 
    /// 
    /// Attaches a test `DB` instance if `attach_db` is `true`.
    pub fn create_test_state(attach_db: bool) -> Result<AppState, AppError> {
        let instance = create_test_instance(attach_db)?;
        let state = AppState {
            instance: Arc::new(Mutex::new(instance)),
        };

        Ok(state)
    }
}
