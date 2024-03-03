use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USSDSession {
    pub id: String,
    pub msisdn: String,
    pub session_id: String,
    pub session_data: String,
    pub session_state: String,
    pub current_screen: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

pub trait USSDSessionTrait {
    fn new() -> Self;
    fn create(&self) -> Result<(), String>;
    fn read(&self) -> Result<(), String>;
    fn update(&self) -> Result<(), String>;
    fn delete(&self) -> Result<(), String>;
}

impl USSDSessionTrait for USSDSession {
    fn new() -> Self {
        USSDSession {
            id: "".to_string(),
            msisdn: "".to_string(),
            session_id: "".to_string(),
            session_data: "".to_string(),
            session_state: "".to_string(),
            current_screen: "".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
            deleted_at: "".to_string(),
        }
    }

    fn create(&self) -> Result<(), String> {
        println!("Creating USSD session");
        Ok(())
    }

    fn read(&self) -> Result<(), String> {
        println!("Reading USSD session");
        Ok(())
    }

    fn update(&self) -> Result<(), String> {
        println!("Updating USSD session");
        Ok(())
    }

    fn delete(&self) -> Result<(), String> {
        println!("Deleting USSD session");
        Ok(())
    }
}