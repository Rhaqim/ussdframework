use crate::{debug, error, info, types::FunctionMap, USSDMenu};

use std::time::Duration;

use super::{ScreenType, SessionCache, USSDAction, USSDRequest, USSDResponse, USSDSession};

/// Sessions idle longer than this are treated as expired and restarted.
const SESSION_TIMEOUT: Duration = Duration::from_secs(5 * 60);

/// Entry point for processing USSD requests.
pub fn process_request(
    request: &USSDRequest,
    session_cache: &Box<dyn SessionCache>,
    screens: &USSDMenu,
    function_map: &FunctionMap,
) -> USSDResponse {
    // Build a generic error response for catastrophic failures.
    let mut response = USSDResponse {
        msisdn: request.msisdn.clone(),
        session_id: request.session_id.clone(),
        end_session: true,
        message: "Service unavailable. Please try again later.".to_string(),
    };

    // Resolve the initial screen — return an error response instead of panicking.
    let initial_screen = match screens.get_initial_screen() {
        Some((name, _)) => name,
        None => {
            error!("No Initial screen found in the loaded menu — check your configuration");
            return response;
        }
    };

    // Generate or retrieve the session.
    let mut session =
        USSDSession::get_or_create_session(request, &initial_screen, session_cache);

    // Restart timed-out sessions.
    if session.has_timed_out(SESSION_TIMEOUT) {
        info!("Session {} timed out — restarting", session.session_id);
        session.restart(&initial_screen);
    }

    response.end_session = session.end_session;
    response.message = "Something went wrong, please try again later".to_string();

    session.display_screen_history();

    let mut current_screen = session.current_screen.clone();

    loop {
        if let Some(screen) = screens.menus.get(&current_screen) {
            info!(
                "\nRunning for {}\nScreen Type: {:?}\nRequest : {:?}\n",
                current_screen, screen.screen_type, request
            );

            match screen.screen_type {
                ScreenType::Initial => {
                    // The initial screen is a pass-through: always advance to
                    // default_next_screen regardless of input.  It is never
                    // interactive and has no display output, so back/home input
                    // handling does not apply here.
                    session.current_screen = screen.default_next_screen.clone();
                }

                ScreenType::Function | ScreenType::Router => {
                    screen.execute(&mut session, request, &screens.services, function_map);
                }

                _ => {
                    let current_screen_displayed = session
                        .displayed
                        .entry(current_screen.clone())
                        .or_insert(false);

                    if !*current_screen_displayed {
                        debug!("Displaying message for screen: {}", current_screen);

                        response.message = screen.display(&mut session).unwrap_or_else(|| {
                            error!(
                                "Failed to display message for screen: {}",
                                current_screen
                            );
                            "Something went wrong, please stop".to_string()
                        });

                        response.end_session = session.end_session;

                        session.displayed.insert(current_screen.clone(), true);
                        session.current_screen = current_screen.clone();
                        session.update_session(session_cache);

                        break;
                    } else {
                        debug!("Executing action for screen: {}", current_screen);

                        screen.execute(&mut session, request, &screens.services, function_map);

                        session.displayed.remove(&current_screen);
                    }
                }
            }

            current_screen = session.current_screen.clone();
            continue;
        } else {
            break;
        }
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        ussd_menu::USSDMenu,
        ussd_screens::{ScreenType, USSDMenuItems, USSDScreen},
        ussd_service::USSDService,
        ussd_session::InMemorySessionStore,
    };
    use crate::types::{FunctionMap, USSDData};
    use std::collections::HashMap;

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn cache() -> Box<dyn SessionCache> {
        Box::new(InMemorySessionStore::new())
    }

    fn request(session_id: &str, input: &str) -> USSDRequest {
        USSDRequest {
            msisdn: "+254700000000".to_string(),
            input: input.to_string(),
            session_id: session_id.to_string(),
            service_code: "*123#".to_string(),
            language: "en".to_string(),
        }
    }

    /// Minimal two-screen menu: Initial → Menu.
    fn simple_menu() -> USSDMenu {
        let mut menus: HashMap<String, USSDScreen> = HashMap::new();
        let services: HashMap<String, USSDService> = HashMap::new();

        menus.insert("InitialScreen".to_string(), USSDScreen {
            screen_type: ScreenType::Initial,
            default_next_screen: "MainMenu".to_string(),
            ..Default::default()
        });

        let mut items = HashMap::new();
        items.insert("ExitOpt".to_string(), USSDMenuItems {
            option: "1".to_string(),
            display_name: "Exit".to_string(),
            next_screen: "ExitScreen".to_string(),
        });
        menus.insert("MainMenu".to_string(), USSDScreen {
            screen_type: ScreenType::Menu,
            text: {
                let mut m = HashMap::new();
                m.insert("default".to_string(), "Main Menu\n1. Exit".to_string());
                crate::core::ussd_screens::ScreenText(m)
            },
            default_next_screen: "ExitScreen".to_string(),
            menu_items: Some(items),
            ..Default::default()
        });
        menus.insert("ExitScreen".to_string(), USSDScreen {
            screen_type: ScreenType::Quit,
            text: {
                let mut m = HashMap::new();
                m.insert("default".to_string(), "Goodbye!".to_string());
                crate::core::ussd_screens::ScreenText(m)
            },
            default_next_screen: "MainMenu".to_string(),
            ..Default::default()
        });

        USSDMenu { menus, services }
    }

    // ── Tests ─────────────────────────────────────────────────────────────────

    #[test]
    fn first_request_shows_main_menu() {
        let menu = simple_menu();
        let req = request("proc-1", "");
        let function_map = FunctionMap::new();
        let cache = cache();

        let resp = process_request(&req, &cache, &menu, &function_map);

        assert!(!resp.end_session);
        assert!(resp.message.contains("Main Menu"), "got: {}", resp.message);
    }

    #[test]
    fn selecting_exit_ends_session() {
        let menu = simple_menu();
        let cache = cache();
        let function_map = FunctionMap::new();

        // First call — display menu
        let req1 = request("proc-2", "");
        process_request(&req1, &cache, &menu, &function_map);

        // Second call — select option 1 (Exit)
        let req2 = request("proc-2", "1");
        let resp = process_request(&req2, &cache, &menu, &function_map);

        assert!(resp.end_session);
        assert!(resp.message.contains("Goodbye!"), "got: {}", resp.message);
    }

    #[test]
    fn missing_initial_screen_returns_error_response() {
        let mut menu = USSDMenu::new();
        // Deliberately omit an Initial screen.
        menu.menus.insert("MainMenu".to_string(), USSDScreen {
            screen_type: ScreenType::Menu,
            ..Default::default()
        });
        let cache = cache();
        let function_map = FunctionMap::new();
        let req = request("proc-3", "");

        let resp = process_request(&req, &cache, &menu, &function_map);

        assert!(resp.end_session);
        assert!(resp.message.contains("unavailable") || resp.message.contains("wrong"),
            "got: {}", resp.message);
    }

    #[test]
    fn function_screen_stores_result_in_session() {
        // Build a menu with a Function screen backed by a Rust function.
        let mut menu = USSDMenu::new();
        let mut services: HashMap<String, USSDService> = HashMap::new();
        services.insert("greet".to_string(), USSDService {
            function_name: "greet".to_string(),
            function_url: None,
            data_key: "greeting".to_string(),
            service_code: None,
        });

        menu.menus.insert("InitialScreen".to_string(), USSDScreen {
            screen_type: ScreenType::Initial,
            default_next_screen: "FnScreen".to_string(),
            ..Default::default()
        });
        menu.menus.insert("FnScreen".to_string(), USSDScreen {
            screen_type: ScreenType::Function,
            function: Some("greet".to_string()),
            default_next_screen: "ResultScreen".to_string(),
            ..Default::default()
        });
        menu.menus.insert("ResultScreen".to_string(), USSDScreen {
            screen_type: ScreenType::Quit,
            text: {
                let mut m = HashMap::new();
                m.insert("default".to_string(), "{{greeting}}".to_string());
                crate::core::ussd_screens::ScreenText(m)
            },
            default_next_screen: "InitialScreen".to_string(),
            ..Default::default()
        });
        menu.services = services;

        let mut function_map = FunctionMap::new();
        function_map.insert(
            "greet".to_string(),
            |_session, _url| USSDData::Str("Hello, world!".to_string()),
        );

        let cache = cache();

        // First call: Initial → FnScreen (executes function) → ResultScreen (displays)
        let req = request("proc-fn-1", "");
        let resp = process_request(&req, &cache, &menu, &function_map);

        assert!(resp.end_session);
        assert!(resp.message.contains("Hello, world!"), "got: {}", resp.message);
    }
}
