use std::collections::HashMap;

use crate::{
    builder::{Database, DatabaseManager, MenuItem, RouterOption, ScreenModel, ServiceModel},
    core::{
        ussd_screens::{USSDMenuItems, USSDRouterOption},
        USSDMenu,
    },
    debug, error, info,
};

/// Loads a `USSDMenu` from a JSON file and populates the database with the menu data.
///
/// This function attempts to load a `USSDMenu` from the specified JSON file or defaults to
/// "menu.json" if no file path is provided. It then initializes the `DatabaseManager` and
/// populates the database with the services, screens, menu items, and router options
/// from the loaded menu.
///
/// # Arguments
///
/// * `file_path` - An optional file path to the JSON file containing the menu data.
///
/// # Panics
///
/// This function will log an error if it fails to load the menu from the JSON file.
///
/// # Examples
///
/// ```
/// from_json(Some("path/to/menu.json"));
/// // This will load the menu from the specified file and populate the database.
/// ```
///
/// ```
/// from_json(None);
/// // This will load the menu from "menu.json" and populate the database.
/// ```
pub fn from_json(file_path: Option<&str>) {
    info!("Loading menu from JSON file: {:?}", file_path);

    let menu = USSDMenu::load_from_json(file_path.unwrap_or("menu.json"));

    match menu {
        Ok(m) => {
            info!("Menu loaded successfully");

            // Initialize the database manager
            let mut db = DatabaseManager::new();

            // Initialize the models
            let mut services: Vec<ServiceModel> = Vec::new();

            let mut screens: Vec<ScreenModel> = Vec::new();

            let mut menu_items: Vec<MenuItem> = Vec::new();

            let mut router_options: Vec<RouterOption> = Vec::new();

            // Load the services and screens
            for (name, service) in m.services {
                services.push(ServiceModel::from_ussd_service(name, service));
            }

            for (name, screen) in m.menus {
                screens.push(ScreenModel::from_ussd_menu(name.clone(), screen.clone()));

                let mut items: HashMap<String, USSDMenuItems> = HashMap::new();
                let mut routes: Vec<USSDRouterOption> = Vec::new();

                match screen.menu_items {
                    Some(i) => items = i,
                    None => debug!("No menu items found for screen {}", name),
                }

                match screen.router_options {
                    Some(r) => routes = r,
                    None => debug!("No router options found for screen {}", name),
                }

                for (menu_name, menu_item) in items {
                    let menu_item_db =
                        MenuItem::from_ussd_menu_item(name.clone(), menu_name, menu_item);

                    menu_items.push(menu_item_db);
                }

                for route in routes {
                    let route_db = RouterOption::from_ussd_router_option(name.clone(), route);

                    router_options.push(route_db);
                }
            }

            // Create the services and screens
            for service in services {
                let _ = db.create(service);
            }

            for screen in screens {
                let _ = db.create(screen);
            }

            for menu_item in menu_items {
                let _ = db.create(menu_item);
            }

            for route in router_options {
                let _ = db.create(route);
            }

            info!("Menu loaded successfully");
        }
        Err(e) => error!("Failed to load menu: {}", e),
    }
}

/// Saves a `USSDMenu` to a JSON file.
///
/// This function attempts to save the provided `USSDMenu` to the specified JSON file.
/// If no file path is provided, it defaults to "menu.json".
///
/// # Arguments
///
/// * `file_path` - An optional file path where the menu should be saved.
/// * `menu` - The `USSDMenu` object to be saved.
///
/// # Panics
///
/// This function will print an error message if it fails to save the menu to the JSON file.
///
/// # Examples
///
/// ```
/// let menu = build();
/// to_json(Some("path/to/menu.json"), menu);
/// // This will save the menu to the specified file.
/// ```
///
/// ```
/// let menu = build();
/// to_json(None, menu);
/// // This will save the menu to "menu.json".
/// ```
pub fn to_json(file_path: Option<&str>, menu: USSDMenu) {
    match file_path {
        Some(path) => {
            let res = menu.save_to_json(path);
            match res {
                Ok(_) => println!("Menu saved to {}", path),
                Err(e) => println!("Failed to save menu: {}", e),
            }
        }
        None => {
            let res = menu.save_to_json("menu.json");
            match res {
                Ok(_) => println!("Menu saved to menu.json"),
                Err(e) => println!("Failed to save menu: {}", e),
            }
        }
    }
}
