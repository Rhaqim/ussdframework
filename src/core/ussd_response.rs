use serde::{Deserialize, Serialize};

// Define structure for USSD response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct USSDResponse {
    pub msisdn: String,
    pub session_id: String,
    pub end_session: bool,
    pub message: String,
}
