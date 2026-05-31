use serde::{Deserialize, Serialize};

/// Represents a USSD request.
///
/// The `USSDRequest` struct represents a request in the context of Unstructured Supplementary
/// Service Data (USSD) communication. It encapsulates various request parameters, including
/// the mobile subscriber ISDN (MSISDN), input data, session ID, service code, and language.
///
/// This struct is used to capture user input and metadata associated with a USSD session.
///
/// # Fields
///
/// * `msisdn`: A string representing the mobile subscriber ISDN (MSISDN) associated with the request.
/// * `input`: A string representing the user input provided in the request.
/// * `session_id`: A string representing the unique identifier of the session associated with the request.
/// * `service_code`: A string representing the service code associated with the request.
/// * `language`: A string representing the language preference of the user.
///
/// # Derives
///
/// The `USSDRequest` struct derives `Debug`, `Clone`, `Serialize`, and `Deserialize` traits
/// to enable debugging, cloning, serialization, and deserialization of request instances.
///
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct USSDRequest {
    pub msisdn: String,
    pub input: String,
    pub session_id: String,
    pub service_code: String,
    pub language: String,
}

// ── Gateway-specific adapters ─────────────────────────────────────────────────

/// Africa's Talking USSD gateway request format.
///
/// AT sends `Content-Type: application/x-www-form-urlencoded` with these fields.
/// The `text` field is **cumulative** — it contains the full `*`-delimited input
/// chain for the session (e.g. `"1*2*3"` after three interactions). Only the last
/// segment is the user's most recent input.
///
/// Reference: <https://developers.africastalking.com/docs/ussd/handle_sessions>
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AfricasTalkingRequest {
    pub session_id: String,
    pub phone_number: String,
    pub network_code: String,
    pub service_code: String,
    /// Full input chain, e.g. `""`, `"1"`, `"1*2"`, `"1*2*3"`
    pub text: String,
}

impl AfricasTalkingRequest {
    /// Convert into the framework's internal `USSDRequest`.
    ///
    /// Extracts only the **last** `*`-delimited segment of `text` as the current
    /// user input, matching the framework's expectation of one input per request.
    pub fn into_ussd_request(self) -> USSDRequest {
        let input = self
            .text
            .split('*')
            .last()
            .unwrap_or("")
            .to_string();

        USSDRequest {
            msisdn: self.phone_number,
            input,
            session_id: self.session_id,
            service_code: self.service_code,
            language: "en".to_string(), // AT does not send a language field
        }
    }
}
