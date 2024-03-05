use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct USSDResponse {
    pub session_id: String,
    pub msisdn: String,
     pub request_id: String,
    pub message: String,
}

impl USSDResponse {
    // pub fn new(session_id: String, msisdn: String, request_id: String, message: String) -> Self {
    //     USSDResponse {
    //         session_id,
    //         msisdn,
    //         request_id,
    //         message,
    //     }
    // }

    // pub fn format_message(&self) -> String {
    //     format!("{}{}", self.message, "\n")
    // }
}
