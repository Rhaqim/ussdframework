use serde::{Deserialize, Serialize};

// Define structure for USSD response
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct USSDResponse {
    pub msisdn: String,
    pub session_id: String,
    pub end_session: bool,
    pub message: String,
}

impl USSDResponse {
    /// Render as the plain-text format expected by Africa's Talking and most
    /// other USSD gateways: `CON <message>` (session continues) or
    /// `END <message>` (session ends).
    pub fn to_gateway_string(&self) -> String {
        if self.end_session {
            format!("END {}", self.message)
        } else {
            format!("CON {}", self.message)
        }
    }
}
