use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use crate::helper::stack::Stack;

use super::{ussd_menu::USSDMenu, ussd_screen::UssdAction, ussd_session::USSDSession};

// Define the USSDRequest struct
pub struct USSDRequest {
    pub session: USSDSession,
    pub menu: USSDMenu,
    pub timeout_duration: Duration,
}

impl USSDRequest {
    // Create a new USSDRequest
    pub fn new(session_id: String, menu: USSDMenu, timeout_duration: Duration) -> Self {
        let (start_screen, _) = menu.get_initial_screen();

        USSDRequest {
            session: USSDSession {
                session_id,
                data: HashMap::new(),
                current_screen: start_screen,
                visited_screens: Stack::new(),
                last_interaction_time: SystemTime::now(),
                end_session: false,
            },
            menu,
            timeout_duration,
        }
    }

    // Handle USSD request
    pub fn handle_ussd_request(&mut self, input: &str) -> Option<String> {
        let (start_screen, _) = self.menu.get_initial_screen();
        if self.session.has_timed_out(self.timeout_duration) {
            self.session.restart(&start_screen);
        }

        if let Some(screen) = self.menu.menus.get(&self.session.current_screen) {
            let next_screen = screen.execute(&mut self.session, input, &self.menu.services);
            self.session.last_interaction_time = SystemTime::now();
            next_screen
        } else {
            None // Invalid screen
        }
    }
}
