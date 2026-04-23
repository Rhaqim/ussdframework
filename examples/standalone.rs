/// Standalone example — no `menubuilder` feature required.
///
/// The developer provides the `USSDMenu` directly (loaded from a JSON file here,
/// but it can be built in code or loaded from any source).  `UssdApp::run()` handles
/// each request synchronously without starting any HTTP server.
use ussdframework::prelude::*;

mod functions;

fn main() {
    // Load the menu definition from a JSON file.
    // In a real application you would build this from your own data source.
    let menu = USSDMenu::load_from_json("examples/data/menu.json")
        .expect("Failed to load menu.json");

    // Create the app and register the function handlers.
    let mut app = UssdApp::new(true, None);
    app.register_functions(functions::get_functions());

    // Simulate a session by processing a sequence of user inputs.
    let session_id = "demo-session-001".to_string();
    let msisdn = "+2348100000000".to_string();
    let service_code = "*123#".to_string();

    let inputs = vec![
        "",   // Initial dial — no input yet
        "1",  // Balance Inquiry
        "6",  // Exit
    ];

    for input in inputs {
        let request = USSDRequest {
            session_id: session_id.clone(),
            service_code: service_code.clone(),
            msisdn: msisdn.clone(),
            input: input.to_string(),
            language: "en".to_string(),
        };

        let response = app.run(request, menu.clone());
        println!(">>> Input: {:?}", input);
        println!("    {}", response.message);
        println!();
    }
}
