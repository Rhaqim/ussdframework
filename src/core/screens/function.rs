use crate::core::UssdSession;

pub fn function_handler(
    session: &mut UssdSession,
    function_name: &str,
    default_next_screen: &String,
) -> Option<String> {
    match function_name {
        "get_balance" => {
            session.current_screen = default_next_screen.clone();
            Some(default_next_screen.clone())
        }
        "get_mini_statement" => {
            session.current_screen = default_next_screen.clone();
            Some(default_next_screen.clone())
        }
        _ => {
            session.current_screen = default_next_screen.clone();
            Some(default_next_screen.clone())
        }
    }
}
