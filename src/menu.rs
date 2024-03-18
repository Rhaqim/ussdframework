use config::Config;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::core::{
    ussd_screens::{Screen, ScreenType},
    ussd_service::USSDService,
};

// Define a structure to hold the USSD menu data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDMenu {
    pub menus: HashMap<String, Screen>,
    pub services: HashMap<String, USSDService>,
}

impl USSDMenu {
    // Load menu structure from JSON file
    pub fn load_from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // let contents = include_str!("../data/menu.json");
        let menu: USSDMenu = serde_json::from_str(&contents)?;
        Ok(menu)
    }

    // Save menu structure to JSON file
    pub fn _save_to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = serde_json::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    // load menu from config
    pub fn _load_from_config(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let menu: USSDMenu = config.get("menu")?;
        Ok(menu)
    }

    // Get the Intial screen
    pub fn get_initial_screen(&self) -> (String, &Screen) {
        for (screen_name, screen) in self.menus.iter() {
            if let ScreenType::Initial = screen.screen_type {
                return (screen_name.clone(), screen);
            }
        }
        panic!("No initial screen found!");
    }

    pub fn belonging_to(&self, service_code: &str) -> USSDMenu {
        let mut menus = HashMap::new();
        let mut services = HashMap::new();

        for (name, screen) in self.menus.iter() {
            match screen.service_code {
                Some(ref code) if code == service_code => {
                    menus.insert(name.clone(), screen.clone());
                }
                None => {}
                _ => {}
            }
        }

        for (name, service) in self.services.iter() {
            match service.service_code {
                Some(ref code) if code == service_code => {
                    services.insert(name.clone(), service.clone());
                }
                None => {}
                _ => {}
            }
        }

        USSDMenu { menus, services }
    }

    // pub fn builder(service_code: &str, connection: DbConnection) -> MenuBuilder {
    //     MenuBuilder::new(service_code, connection)
    // }
}

#[cfg(feature = "menubuilder")]
pub mod menubuilder {
    use crate::menu::USSDMenu;
    use crate::ussd_screens::Screen;
    use crate::ussd_service::USSDService;
    use diesel::{self, prelude::*};

    pub trait MenuBuilderTrait {
        fn new(service_code: &str, connection: Option<DbConnection>) -> Self;
        fn add_screen(&self, screen: Screen);
        fn add_service(&self, service: USSDService);
        fn build(&self) -> USSDMenu;
    }

    pub struct MenuBuilder {
        pub service_code: String,
        pub connection: Option<DbConnection>,
    }

    impl MenuBuilderTrait for MenuBuilder {
        fn new(service_code: &str, connection: Option<DbConnection>) -> Self {
            MenuBuilder {
                service_code: service_code.to_string(),
                connection,
            }
        }

        fn add_screen(&self, screen: Screen) {
            if let Some(ref conn) = self.connection {
                diesel::insert_into(screens::table)
                    .values(&screen)
                    .execute(conn)
                    .expect("Error saving new screen");
            } else {
                panic!("No database connection provided");
            }
        }

        fn add_service(&self, service: USSDService) {
            if let Some(ref conn) = self.connection {
                // Add the service to the database
                diesel::insert_into(services::table)
                    .values(&service)
                    .execute(conn)
                    .expect("Error saving new service");
            } else {
                panic!("No database connection provided");
            }
        }

        fn build(&self) -> USSDMenu {
            let mut menus = HashMap::new();
            let mut services = HashMap::new();

            if let Some(ref conn) = self.connection {
                // Add the screens and services to the menu
                for screen in Screen::belonging_to(&self.service_code)
                    .load::<Screen>(conn)
                    .expect("Error loading screens")
                {
                    menus.insert(screen.name.clone(), screen);
                }

                for service in USSDService::belonging_to(&self.service_code)
                    .load::<USSDService>(conn)
                    .expect("Error loading services")
                {
                    services.insert(service.name.clone(), service);
                }
            } else {
                panic!("No database connection provided");
            }

            USSDMenu { menus, services }
        }
    }
}

#[cfg(not(feature = "menubuilder"))]
pub mod menubuilder {}
