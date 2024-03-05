// src/examples/simple_ussd_gateway.rs

// Include the necessary imports
use ussdframework::core::{USSDGateway, USSDSession};
use ussdframework::config::USSDConfig;
use ussdframework::types::HashStrAny;

fn main() {
    // Create a USSD config
    let config = USSDConfig::new();

    // Define USSD services
    // config.add_service("ServiceName".to_string(), USSDServiceConfig { ... });

    // Create a USSD gateway
    let gateway = USSDGateway::new("functions_path".to_string(), "menu_source".to_string());

    // Initialize the USSD gateway
    gateway.initial();
}
