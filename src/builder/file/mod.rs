pub mod json;

use std::collections::HashMap;

pub use json::{from_json, to_json};

use crate::core::USSDMenu;

use super::{Database, DatabaseManager, ScreenModel, ServiceModel};

/// Builds and returns a `USSDMenu` by fetching and converting screen and service data.
///
/// This function initializes a new `DatabaseManager` and uses it to retrieve screen and service
/// data. The retrieved data is then converted into USSD-compatible formats and stored in
/// hash maps. These hash maps are used to construct a `USSDMenu` object which is then returned.
///
/// # Panics
///
/// This function will panic if it fails to retrieve screens or services from the database.
///
/// # Examples
///
/// ```
/// let ussd_menu = build();
/// // `ussd_menu` now contains the USSD screens and services.
/// ```
///
/// # Returns
///
/// A `USSDMenu` containing the converted screens and services.
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
