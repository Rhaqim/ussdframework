use crate::core::UssdSession;

pub fn quit_handler(
    session: &mut UssdSession,
    _input: &str,
    _default_next_screen: &String,
) -> Option<String> {
    session.end_session = true;
    None
}
