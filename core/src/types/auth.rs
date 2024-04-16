use super::Config;

#[derive(Clone)]
pub(crate) struct User {
    pub id: String,
    pub is_logged: bool,
}

impl User {
    
    pub(crate) fn validate(&self) -> bool {
        false
    }

    pub(crate) fn logout(&self){
        // TODO: Close connection
    }

    /// Saves this `Config` for user and create new connection using the `Config`.
    pub(crate) fn save_new_config(&self, config: &Config) {
        
    }
}