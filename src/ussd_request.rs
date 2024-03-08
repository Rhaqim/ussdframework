use serde::{Deserialize, Serialize};

// Define structure for USSD request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USSDRequest {
    pub msisdn: String,
    pub input: String,
    pub session_id: String,
    pub service_code: String,
    pub language: String,
}
