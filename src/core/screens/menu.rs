use std::collections::HashMap;

use crate::core::{MenuItems, USSDSession};

pub fn menu_handler(session: &mut USSDSession, input: &str, menu_items: &HashMap<String, MenuItems>, default_next_screen: &String) -> Option<String> {
    // iterate over the menu_items and give each item an index within the bounds of the menu_items_len
    let menu_items_len = menu_items.len();

    // if input is within the bounds of the menu_items_len, return the next screen
    if let Ok(input) = input.parse::<usize>() {
        if input > 0 && input <= menu_items_len {
            let next_screen = menu_items.values().nth(input - 1).unwrap().default_next_screen.clone();
            session.current_screen = next_screen.clone();
            return Some(next_screen);
        }
    }

    // if input is not within the bounds of the menu_items_len, return the current screen
    session.current_screen = default_next_screen.clone();
    Some(default_next_screen.clone())
}