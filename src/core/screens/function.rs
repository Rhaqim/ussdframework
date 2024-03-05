use crate::core::UssdSession;

pub fn function_handler(
    session: &mut UssdSession,
    function: &str,
    _data_key: &String,
    default_next_screen: &String,
) -> Option<String> {
    match function {
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
