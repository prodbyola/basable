use std::env;

pub(crate) fn get_env(key: &str) -> String {
    env::var(key).unwrap()
}