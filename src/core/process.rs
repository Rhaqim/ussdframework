use crate::{debug, error, info, USSDMenu};

use super::{ScreenType, SessionCache, USSDAction, USSDRequest, USSDResponse, USSDSession};

/// Entry point for processing USSD requests.
///
/// # Arguments
///
/// * `request` - The USSD request.
/// * `functions_path` - The path to the functions used by the USSD application.
/// * `session_cache` - The session cache implementation used by the USSD application.
/// * `screens` - The USSD menu screens.
///
/// # Returns
///
/// The USSD response.
pub fn process_request(
    request: &USSDRequest,
    functions_path: &String,
    session_cache: &Box<dyn SessionCache>,
    screens: &USSDMenu,
) -> USSDResponse {
    // Get the initial screen
    let (initial_screen, _) = screens.get_initial_screen();

    // Generate or retrieve the session
    let mut session = USSDSession::get_or_create_session(request, &initial_screen, session_cache);

    // Create a response object
    let mut response: USSDResponse = USSDResponse {
        msisdn: request.msisdn.clone(),
        session_id: request.session_id.clone(),
        message: "Something went wrong, please try again later".to_string(),
    };

    // Display screen history
    session.display_screen_history();

    let mut current_screen = session.current_screen.clone();

    loop {
        if let Some(screen) = screens.menus.get(&current_screen) {
            info!(
                "\nRunning for {}\nScreen Type: {:?}\nRequest : {:?}\n",
                current_screen, screen.screen_type, request
            );

            // Execute the screen action for Function, Router, and Initial screen types
            // They contain no display message
            // They are used to execute a function, route to another screen, or set the initial screen
            // The next screen is set based on the action
            match screen.screen_type {
                ScreenType::Function | ScreenType::Router | ScreenType::Initial => {
                    screen.execute(
                        &mut session,
                        request,
                        functions_path.clone(),
                        &screens.services,
                    );
                }

                // Display the screen message and execute the screen action for Menu and Input screen types
                // They contain a display message
                // The next screen is set based on the action
                // It checks if the current screen has been displayed
                // If not, it displays the message and sets the current screen as displayed and also routes back to the current screen
                _ => {
                    let current_screen_displayed = session
                        .displayed
                        .entry(current_screen.clone())
                        .or_insert(false);

                    if !*current_screen_displayed {
                        debug!("Displaying message for screen: {}", current_screen);

                        response.message = screen.display(&session).unwrap_or_else(|| {
                            error!("Failed to display message for screen: {} please ensure the screen has a message", current_screen);
                            "Something went wrong, please stop".to_string()
                        });

                        session.displayed.insert(current_screen.clone(), true);
                        session.current_screen = current_screen.clone();
                        session.update_session(session_cache);

                        break;
                    } else {
                        debug!("Executing action for screen: {}", current_screen);

                        screen.execute(
                            &mut session,
                            request,
                            functions_path.clone(),
                            &screens.services,
                        );

                        // remove from displayed
                        session.displayed.remove(&current_screen);
                    }
                }
            }
            // request.session_data = session.data.clone();
            current_screen = session.current_screen.clone();
            continue;
        } else {
            break;
        }
    }

    return response;
}
