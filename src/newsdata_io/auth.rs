use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    api_key: String,
}
impl Clone for Auth {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
        }
    }
}
impl Auth {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }
}
