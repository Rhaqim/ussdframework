pub mod json;

use std::collections::HashMap;

pub use json::{from_json, to_json};

use crate::core::USSDMenu;
use crate::info;

use super::{Database, DatabaseManager, ScreenModel, ServiceModel};

/// Builds and returns a `USSDMenu` by fetching screen and service data from the database.
///
/// If the database is empty and `json_seed` is provided, the JSON file at that path is
/// loaded into the database first. This lets callers bootstrap from a JSON definition
/// without a separate import step.
///
/// # Arguments
///
/// * `json_seed` — Optional path to a JSON file used to seed the database when empty.
pub fn build(json_seed: Option<&str>) -> USSDMenu {
    let mut db = DatabaseManager::new();

    let mut menus = HashMap::new();
    let mut services = HashMap::new();

    let screens: Vec<ScreenModel> = db.get_many().unwrap_or_default();

    if screens.is_empty() {
        if let Some(path) = json_seed {
            info!("Database is empty — seeding from '{}'", path);
            from_json(Some(path));
            // Re-open so we pick up the newly inserted rows.
            db = DatabaseManager::new();
        }
    }

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
