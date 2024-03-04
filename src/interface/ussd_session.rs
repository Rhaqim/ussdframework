use serde::{Deserialize, Serialize};

use std::time::{Duration, SystemTime};

use crate::helper::stack::Stack;

#[derive(Debug, Deserialize, Serialize)]
pub struct UssdSession {
    pub session_id: String,
    pub current_screen: String,
    pub visited_screens: Stack<String>,
    pub last_interaction_time: SystemTime,
    // Add any other session-related data here
}

impl UssdSession {
    // Check if session has timed out
    pub fn has_timed_out(&self, timeout_duration: Duration) -> bool {
        self.last_interaction_time.elapsed().unwrap_or_default() > timeout_duration
    }

    // Restart the session
    pub fn restart(&mut self, initial_screen: &str) {
        // clear visited screens
        self.visited_screens.clear();
        self.current_screen = initial_screen.to_string();
        self.last_interaction_time = SystemTime::now();
        // Reset any other session-related data as needed
    }
}
