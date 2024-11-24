use std::env::{self, VarError};

pub(crate) fn get_env(key: &str) -> Result<String, VarError> {
    env::var(key)
}