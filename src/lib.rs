mod log;
mod menu;
pub mod prelude;
pub mod types;
mod ussd_request;
mod ussd_response;
mod ussd_screens;
mod ussd_service;
mod ussd_session;
mod utils;

extern crate serde;

use prelude::USSDMenu;
use ussd_request::USSDRequest;
use ussd_response::USSDResponse;
use ussd_screens::process_request;
use ussd_session::SessionCache;

/// Represents a USSD application.
pub struct UssdApp {
    functions_path: String,
    pub session_cache: Box<dyn SessionCache>,
}

impl UssdApp {
    /// Creates a new instance of `UssdApp`.
    ///
    /// # Arguments
    ///
    /// * `functions_path` - The path to the functions used by the USSD application.
    /// * `session_cache` - The session cache implementation used by the USSD application.
    ///
    /// # Returns
    ///
    /// A new instance of `UssdApp`.
    pub fn new(
        functions_path: String,
        built_in_session_manager: bool,
        session_manager: Option<Box<dyn SessionCache>>,
    ) -> UssdApp {
        let session_cache: Box<dyn SessionCache>;

        if built_in_session_manager || session_manager.is_none() {
            session_cache = Box::new(ussd_session::InMemorySessionStore::new());
        } else {
            session_cache = session_manager.unwrap();
        }

        UssdApp {
            functions_path,
            session_cache,
        }
    }

    /// Runs the USSD application with the given request and screens.
    ///
    /// # Arguments
    ///
    /// * `request` - The USSD request.
    /// * `screens` - The USSD menu screens.
    ///
    /// # Returns
    ///
    /// The USSD response.
    pub fn run(&self, request: USSDRequest, screens: USSDMenu) -> ussd_response::USSDResponse {
        process_request(
            &request,
            &self.functions_path,
            &self.session_cache,
            &screens,
        )
    }

    /// Displays the menu to the user.
    ///
    /// # Arguments
    ///
    /// * `ussd_response` - The USSD response containing the menu message.
    pub fn display_menu(&self, ussd_response: &USSDResponse) {
        // Display the menu to the user
        println!("{}", ussd_response.message);
    }
}

#[cfg(feature = "menubuilder")]
pub mod menubuilder {
    pub mod menubuilder {
        use crate::menu::USSDMenu;
        use crate::ussd_screens::Screen;
        use crate::ussd_service::USSDService;
        use diesel::{self, prelude::*};

        pub struct MenuBuilder {
            pub service_code: String,
            pub connection: DbConnection,
        }

        impl MenuBuilder {
            pub fn new(service_code: &str, connection: DbConnection) -> Self {
                MenuBuilder {
                    service_code: service_code.to_string(),
                    connection,
                }
            }

            pub fn add_screen(&self, screen: Screen) {
                diesel::insert_into(screens::table)
                    .values(&screen)
                    .execute(&self.connection)
                    .expect("Error saving new screen");
            }

            pub fn add_service(&self, service: USSDService) {
                // Add the service to the database
                diesel::insert_into(services::table)
                    .values(&service)
                    .execute(&self.connection)
                    .expect("Error saving new service");
            }

            pub fn build(&self) -> USSDMenu {
                // Build the USSD menu
                let mut menus = HashMap::new();
                let mut services = HashMap::new();

                // Add the screens and services to the menu
                for screen in Screen::belonging_to(&self.service_code)
                    .load::<Screen>(&self.connection)
                    .expect("Error loading screens")
                {
                    menus.insert(screen.name.clone(), screen);
                }

                for service in USSDService::belonging_to(&self.service_code)
                    .load::<USSDService>(&self.connection)
                    .expect("Error loading services")
                {
                    services.insert(service.name.clone(), service);
                }

                USSDMenu { menus, services }
            }
        }
    }
}

#[cfg(not(feature = "menubuilder"))]
pub struct MenuBuilder {}
