use crate::{core::UssdSession, types::HashStrAny};

pub fn input_handler(
    session: &mut UssdSession,
    input: &str,
    _input_type: &Option<String>,
    input_identifier: &String,
    default_next_screen: &String,
) -> Option<String> {
    session
        .data
        .insert(input_identifier.clone(), HashStrAny::Str(input.to_string()));
    session.current_screen = default_next_screen.clone();
    Some(default_next_screen.clone())
}
