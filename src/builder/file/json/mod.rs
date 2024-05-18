use crate::{
    builder::{
        schema::{MenuItem, RouterOption, Screen as ScreenModel, Service as ServiceModel},
        {Database, DatabaseManager},
    },
    core::USSDMenu,
    {error, info},
};

pub fn from_json(file_path: Option<&str>) {
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

                let items = screen.menu_items.unwrap();
                let routes = screen.router_options.unwrap();

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
