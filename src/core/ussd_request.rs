use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::HashStrAny;

// Define structure for USSD request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USSDRequest {
    pub msisdn: String,
    pub input: String,
    pub session_id: String,
    pub session_data: HashMap<String, HashStrAny>,
    pub service_code: String,
    pub language: String,
}

impl USSDRequest {
    /// Fetches an item from the session data based on the given key.
    pub fn fetch_session_data<'a>(&'a self, key: &str) -> Option<&'a HashStrAny> {
        self.session_data.get(key)
    }
}
