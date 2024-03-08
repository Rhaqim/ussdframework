use std::{collections::HashMap, sync::Mutex};

use ussdframework::prelude::*;

fn main() {
    let session_store = InMemorySessionStore::new();
    let app = UssdApp::new("config/functions".to_string(), Box::new(session_store));

    let content = include_str!("../examples/data/menu.json");
    let menus: USSDMenu = serde_json::from_str(&content).unwrap();

    loop {
        // Define input variable for each iteration
        let mut input = String::new();

        let request = USSDRequest {
            session_id: "1234".to_string(),
            msisdn: "1234".to_string(),
            input: input.clone(), // Use input directly here
            service_code: "1234".to_string(),
            language: "en".to_string(),
        };

        let response = app.run(request, menus.clone());
        app.display_menu(&response);

        // Process user input and update request input for the next iteration
        // For demonstration purposes, let's assume the input is obtained from user input
        println!("Enter your choice:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim().to_string();

        // Break out of the loop if the user chooses to exit
        if trimmed_input == "exit" {
            break;
        }
    }
}

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
