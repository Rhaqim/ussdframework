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

