pub mod json;

use std::collections::HashMap;

pub use json::{from_json, to_json};

use crate::core::USSDMenu;

use super::{Database, DatabaseManager, Screen as ScreenModel, Service as ServiceModel};

pub fn build() -> USSDMenu {
    let mut db = DatabaseManager::new();

    let mut menus = HashMap::new();
    let mut services = HashMap::new();

    let menu: Vec<ScreenModel> = db.get_many().expect("Failed to get screens");

    for m in menu {
        menus.insert(m.name.clone(), m.to_ussd_screen());
    }

    let service: Vec<ServiceModel> = db.get_many().expect("Failed to get services");

    for s in service {
        services.insert(s.name.clone(), s.to_ussd_service());
    }

    USSDMenu { menus, services }
}
