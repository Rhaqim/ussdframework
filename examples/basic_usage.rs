use std::{collections::HashMap, sync::Mutex};

use ussdframework::prelude::*;

fn main() {
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

    let app = UssdApp::new(
        "config/functions".to_string(),
        Box::new(InMemorySessionStore::new()),
    );
    let request = USSDRequest {
        session_id: "1234".to_string(),
        msisdn: "1234".to_string(),
        input: "1".to_string(),
        service_code: "1234".to_string(),
        language: "en".to_string(),
    };

    let content = include_str!("../examples/data/menu.json");
    
    let menus: USSDMenu = serde_json::from_str(&content).unwrap();

    let response = app.run(request, menus);
    app.display_menu(&response);
}
