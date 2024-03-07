extern crate tokio;

mod core;
mod examples;
mod helper;
mod log;
mod types;

use core::{SessionCache, USSDConfig, USSDGateway, USSDSession};
// use redis;
use std::{
    collections::HashMap,
    io::{self, Write},
    sync::Mutex,
};

use crate::core::{USSDMenu, USSDRequest};

#[tokio::main]
async fn main() {
    let functions_path = "src/data/functions";

    pub struct InMemorySessionStore {
        data: Mutex<HashMap<String, String>>,
    }

    impl InMemorySessionStore {
        pub fn new() -> Self {
            Self {
                data: Mutex::new(HashMap::new()),
            }
        }
    }

    impl SessionCache for InMemorySessionStore {
        fn store_session(&self, session: &USSDSession) -> Result<(), String> {
            let mut data = self.data.lock().unwrap();
            data.insert(
                session.session_id.clone(),
                serde_json::to_string(session).unwrap(),
            );
            Ok(())
        }

        fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
            let data = self.data.lock().unwrap();
            match data.get(session_id) {
                Some(session) => Ok(Some(serde_json::from_str(session).unwrap())),
                None => Ok(None),
            }
        }
    }

    let config = USSDConfig {
        functions_path: functions_path.to_string(),
        session_cache: Box::new(InMemorySessionStore::new()),
    };

    let menus = USSDMenu::load_from_json("src/data/menu.json").unwrap();

    let gateway = USSDGateway::new(config, menus);

    fn read_user_input() -> Option<String> {
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            Some(input.trim().to_string())
        } else {
            None
        }
    }

    let input = read_user_input().unwrap();

    let request = USSDRequest {
        input,
        session_id: "123".to_string(),
        msisdn: "123".to_string(),
        request_id: "123".to_string(),
        telco: "Vodafone".to_string(),
        service_code: "123".to_string(),
        country_code: "123".to_string(),
        language: "en".to_string(),
    };

    loop {
        let response = gateway.process_request(request.clone());

        match response {
            Some(response) => {
                println!("Response: {:?}", response);
            }
            None => {
                println!("Invalid input or screen!");
            }
        }
    
    }
}
