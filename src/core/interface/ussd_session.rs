use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use crate::{helper::stack::Stack, types::HashStrAny};

#[derive(Debug, Deserialize, Serialize)]
pub struct UssdSession {
    pub session_id: String,
    pub data: HashMap<String, HashStrAny>,
    pub current_screen: String,
    pub visited_screens: Stack<String>,
    pub last_interaction_time: SystemTime,
    pub end_session: bool,
    // Add any other session-related data here
}

impl UssdSession {
    // Check if session has timed out
    pub fn has_timed_out(&self, timeout_duration: Duration) -> bool {
        self.last_interaction_time.elapsed().unwrap_or_default() > timeout_duration
    }

    // Update the session's last interaction time
    pub fn update_last_interaction_time(&mut self) {
        self.last_interaction_time = SystemTime::now();
    }

    // Restart the session
    pub fn restart(&mut self, initial_screen: &str) {
        // clear visited screens
        self.visited_screens.clear();
        self.current_screen = initial_screen.to_string();
        self.update_last_interaction_time();
        // Reset any other session-related data as needed
    }

    // Display screen history with an arrow pointing to the current screen
    pub fn display_screen_history(&self) {
        for screen in self.visited_screens.iter() {
            print!("{} \u{25B6} ", screen);
        }
    }
}
