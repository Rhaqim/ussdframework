// Define structure for USSD request
#[derive(Debug)]
pub struct USSDRequest {
    pub msisdn: String,
    pub input: String,
    pub session_id: String,
    pub service_code: String,
}
