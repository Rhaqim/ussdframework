use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDResponse {
    pub msisdn: String,
    pub request_id: String,
    pub telco: String,
    pub service_code: String,
    pub country_code: String,
    pub language: String,
    pub message: String,
}
