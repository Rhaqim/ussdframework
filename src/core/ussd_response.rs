use serde::{Deserialize, Serialize};

// Define structure for USSD response
#[derive(Debug, Serialize, Deserialize)]
pub struct USSDResponse {
    pub msisdn: String,
    pub session_id: String,
    pub end_session: bool,
    pub message: String,
}
