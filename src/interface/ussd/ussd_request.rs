use std::error::Error;

use serde::{Deserialize, Serialize};

use super::ussd_session::{USSDSession, USSDSessionTrait};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USSDRequest {
    pub id: String,
    pub msisdn: String,
    pub session_id: String,
    pub session: USSDSession,
    pub input: String,
    pub default_language: String,
    pub use_built_in_session: bool,
    pub timeout: i32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

pub trait USSDRequestTrait {
    fn new(
        msisdn: String,
        session_id: String,
        input: String,
        default_language: String,
        use_built_in_session: bool,
    ) -> Self;
    fn from_json(&mut self, json: &str) -> Result<(), Box<dyn Error>>;
    fn to_json(&self) -> Result<String, Box<dyn Error>>;
    fn forward(&self, handler_name: &str) -> (USSDRequest, String);
}

impl USSDRequestTrait for USSDRequest {
    fn new(msisdn: String,
        _session_id: String,
        input: String,
        default_language: String,
        use_built_in_session: bool,) -> Self {

        let session = USSDSession::new();
        let session_id = session.get_or_create_session_id(&msisdn);

        USSDRequest {
            id: "".to_string(),
            msisdn,
            session_id,
            session,
            input,
            default_language,
            use_built_in_session,
            timeout: 0,
            created_at: "".to_string(),
            updated_at: "".to_string(),
            deleted_at: "".to_string(),
        }
    }

    fn from_json(&mut self, json: &str) -> Result<(), Box<dyn Error>> {
        let request: USSDRequest = serde_json::from_str(json)?;
        self.id = request.id;
        self.msisdn = request.msisdn;
        self.session_id = request.session_id;
        self.input = request.input;
        self.default_language = request.default_language;
        self.use_built_in_session = request.use_built_in_session;
        self.timeout = request.timeout;
        self.created_at = request.created_at;
        self.updated_at = request.updated_at;
        self.deleted_at = request.deleted_at;
        Ok(())
    }

    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }

    fn forward(&self, handler_name: &str) -> (USSDRequest, String) {
        let mut new_request = self.clone();
        new_request.input = "".to_string();

        (new_request, handler_name.to_owned())
    }
}