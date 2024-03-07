use crate::core::USSDSession;

pub fn quit_handler(
    session: &mut USSDSession,
    _input: &str,
    _default_next_screen: &String,
) -> Option<String> {
    session.end_session = true;
    None
}
