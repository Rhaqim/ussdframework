
// Define structure for USSD response
#[derive(Debug)]
pub struct USSDResponse {
    pub msisdn: String,
    pub session_id: String,
    pub message: String,
}
