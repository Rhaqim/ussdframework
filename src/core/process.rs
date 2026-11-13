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
            debug!(
                "Session {}: running screen {} ({:?})",
                session.session_id, current_screen, screen.screen_type
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

    /// Initial → MainMenu (1: Pin) → PinScreen (Input) → Done (Quit)
    fn pin_menu() -> USSDMenu {
        let mut menu = simple_menu();
        let main = menu.menus.get_mut("MainMenu").unwrap();
        main.menu_items.as_mut().unwrap().insert("PinOpt".to_string(), USSDMenuItems {
            option: "2".to_string(),
            display_name: "Enter PIN".to_string(),
            next_screen: "PinScreen".to_string(),
        });
        menu.menus.insert("PinScreen".to_string(), USSDScreen {
            screen_type: ScreenType::Input,
            text: {
                let mut m = HashMap::new();
                m.insert("default".to_string(), "Enter PIN".to_string());
                crate::core::ussd_screens::ScreenText(m)
            },
            input_identifier: Some("pin".to_string()),
            validation_regex: Some(r"^\d{4}$".to_string()),
            default_next_screen: "ExitScreen".to_string(),
            ..Default::default()
        });
        menu
    }

    #[test]
    fn invalid_option_error_does_not_carry_over_to_next_screen() {
        let menu = pin_menu();
        let cache = cache();
        let fm = FunctionMap::new();

        process_request(&request("carry-1", ""), &cache, &menu, &fm);

        let resp = process_request(&request("carry-1", "9"), &cache, &menu, &fm);
        assert!(resp.message.contains("Invalid menu option"), "got: {}", resp.message);
        assert!(resp.message.contains("Main Menu"), "got: {}", resp.message);

        let resp = process_request(&request("carry-1", "2"), &cache, &menu, &fm);
        assert!(resp.message.contains("Enter PIN"), "got: {}", resp.message);
        assert!(!resp.message.contains("Invalid"), "error carried over: {}", resp.message);
    }

    #[test]
    fn invalid_input_error_shown_once() {
        let menu = pin_menu();
        let cache = cache();
        let fm = FunctionMap::new();

        process_request(&request("carry-2", ""), &cache, &menu, &fm);
        process_request(&request("carry-2", "2"), &cache, &menu, &fm);

        let resp = process_request(&request("carry-2", "abc"), &cache, &menu, &fm);
        assert!(resp.message.contains("Invalid input"), "got: {}", resp.message);

        let resp = process_request(&request("carry-2", "1234"), &cache, &menu, &fm);
        assert!(resp.message.contains("Goodbye!"), "got: {}", resp.message);
        assert!(!resp.message.contains("Invalid"), "error carried over: {}", resp.message);
    }

    #[test]
    fn back_after_invalid_option_goes_to_previous_screen() {
        let menu = pin_menu();
        let cache = cache();
        let fm = FunctionMap::new();

        process_request(&request("back-1", ""), &cache, &menu, &fm);
        process_request(&request("back-1", "2"), &cache, &menu, &fm); // → PinScreen
        process_request(&request("back-1", "x"), &cache, &menu, &fm); // invalid, stay

        let resp = process_request(&request("back-1", "0"), &cache, &menu, &fm);
        assert!(resp.message.contains("Main Menu"), "got: {}", resp.message);
    }

    // ── Logging must never expose user input ──────────────────────────────────

    use std::sync::{Mutex as StdMutex, OnceLock};

    static CAPTURED: OnceLock<StdMutex<Vec<String>>> = OnceLock::new();

    struct CaptureLogger;
    impl ::log::Log for CaptureLogger {
        fn enabled(&self, _: &::log::Metadata) -> bool { true }
        fn log(&self, record: &::log::Record) {
            CAPTURED.get_or_init(Default::default).lock().unwrap()
                .push(format!("{}", record.args()));
        }
        fn flush(&self) {}
    }

    #[test]
    fn typed_pin_never_appears_in_logs() {
        static LOGGER: CaptureLogger = CaptureLogger;
        let _ = ::log::set_logger(&LOGGER);
        ::log::set_max_level(::log::LevelFilter::Trace);

        let menu = pin_menu();
        let cache = cache();
        let fm = FunctionMap::new();
        process_request(&request("log-1", ""), &cache, &menu, &fm);
        process_request(&request("log-1", "2"), &cache, &menu, &fm);
        process_request(&request("log-1", "4827"), &cache, &menu, &fm);
        process_request(&request("log-1", ""), &cache, &menu, &fm);

        let logs = CAPTURED.get_or_init(Default::default).lock().unwrap();
        assert!(logs.iter().any(|l| l.contains("log-1")), "logger captured nothing");
        let leaks: Vec<_> = logs.iter().filter(|l| l.contains("4827")).collect();
        assert!(leaks.is_empty(), "PIN leaked into logs: {:?}", leaks);
    }
}
