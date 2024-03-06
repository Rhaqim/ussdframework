use serde::{Deserialize, Serialize};

// Define the USSDRequest struct
#[derive(Debug, Deserialize, Serialize)]
pub struct USSDRequest {
    pub input: String,
    pub msisdn: String,
    pub session_id: String,
    pub request_id: String,
    pub telco: String,
    pub service_code: String,
    pub country_code: String,
    pub language: String,
}

// impl USSDRequest {
//     // Create a new USSDRequest
//     pub fn new(
//         session_id: String,
//         menu: USSDMenu,
//         timeout_duration: Duration,
//         cache: impl SessionCache,
//     ) -> Self {
//         let (initial_screen, _) = menu.get_initial_screen();

//         let session = USSDSession::get_or_create_session(
//             &session_id,
//             &initial_screen,
//             timeout_duration,
//             &cache,
//         );

//         USSDRequest {
//             session,
//             menu,
//             timeout_duration,
//         }
//     }

//     // Handle USSD request
//     pub fn handle_ussd_request(&mut self, input: &str) -> Option<String> {
//         let (initial_screen, _) = self.menu.get_initial_screen();
//         if self.session.has_timed_out(self.timeout_duration) {
//             self.session.restart(&initial_screen);
//         }

//         if let Some(screen) = self.menu.menus.get(&self.session.current_screen) {
//             let next_screen = screen.execute(&mut self.session, input, &self.menu.services);
//             self.session.last_interaction_time = SystemTime::now();
//             next_screen
//         } else {
//             None // Invalid screen
//         }
//     }
// }
