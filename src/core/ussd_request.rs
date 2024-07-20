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
