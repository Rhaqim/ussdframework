use ussdframework::prelude::*;

fn main() {
    let app = UssdApp::new("functions".to_string(), Box::new(HashMapSessionCache::new()));
    let request = USSDRequest {
        session_id: "1234".to_string(),
        msisdn: "1234".to_string(),
        input: "1".to_string(),
    };
    let response = app.run(request);
    app.display_menu(&response);
}