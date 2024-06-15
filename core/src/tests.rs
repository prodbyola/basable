use dotenv::dotenv;
use std::{
    env,
    sync::{Arc, Mutex},
};

use crate::base::{config::Config, foundation::Basable, user::User, AppError};

pub(crate) fn test_create_config() -> Config {
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

pub fn create_instance() -> Result<Basable, AppError> {
    dotenv().ok();
    let user_id = env::var("TEST_USER_ID").unwrap();

    let config = test_create_config();
    let user = User {
        id: user_id.clone(),
        ..User::default()
    };

    let mut bslb = Basable::default();
    bslb.add_user(Arc::new(Mutex::new(user)));

    let conn = Basable::create_connection(&config)?;
    bslb.attach_db(&user_id, conn.unwrap())?;

    Ok(bslb)
}
