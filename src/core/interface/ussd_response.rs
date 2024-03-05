use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct UssdResponse {
    pub session_id: String,
    pub message: String,
}
