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

#[cfg(test)]
mod tests {
    use super::*;

    fn response(end_session: bool, message: &str) -> USSDResponse {
        USSDResponse {
            msisdn: "+254700000000".to_string(),
            session_id: "sess-1".to_string(),
            end_session,
            message: message.to_string(),
        }
    }

    #[test]
    fn to_gateway_string_con_when_session_continues() {
        let r = response(false, "Choose option:");
        assert_eq!(r.to_gateway_string(), "CON Choose option:");
    }

    #[test]
    fn to_gateway_string_end_when_session_ends() {
        let r = response(true, "Goodbye!");
        assert_eq!(r.to_gateway_string(), "END Goodbye!");
    }

    #[test]
    fn default_response_is_not_end_session() {
        let r = USSDResponse::default();
        assert!(!r.end_session);
    }
}
