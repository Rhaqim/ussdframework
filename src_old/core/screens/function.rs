use std::collections::HashMap;

use crate::core::{interface::ussd_service::USSDServiceTrait, USSDService, USSDSession};

pub fn function_handler(
    session: &mut USSDSession,
    function: &str,
    services: &HashMap<String, USSDService>,
    default_next_screen: &String,
) -> Option<String> {
    if let Some(service) = services.get(function) {
        println!("Calling service: {}", function);
        println!("Service: {:?}", service);

        let _ = service.call(session);
        session.current_screen = default_next_screen.clone();
        Some(default_next_screen.clone())
    } else {
        None
    }
}
