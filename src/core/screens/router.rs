use std::collections::HashMap;

use crate::core::UssdSession;

pub fn router_handler(
    session: &mut UssdSession,
    input: &str,
    router_options: &HashMap<String, String>,
    default_next_screen: &String,
) -> Option<String> {
    let next_screen = router_options.get(input);
    match next_screen {
        Some(screen) => {
            session.current_screen = screen.clone();
            Some(screen.clone())
        }
        None => {
            session.current_screen = default_next_screen.clone();
            Some(default_next_screen.clone())
        }
    }
}
