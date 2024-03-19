use std::collections::HashMap;
use std::error::Error;

use diesel::{deserialize::FromSql, prelude::*, sqlite::SqliteValue};
use serde::{Deserialize, Serialize};

use crate::database::{Database, DatabaseManager};
use diesel::sqlite::Sqlite;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ScreenType {
    Initial,
    Menu,
    Input,
    Function,
    Router,
    Quit,
}

// Define structure for a screen
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Screen {
    pub text: String,
    pub screen_type: ScreenType,
    pub default_next_screen: String,
    #[serde(default)]
    pub service_code: Option<String>,
    #[serde(default)]
    pub menu_items: Option<HashMap<String, MenuItem>>,
    #[serde(default)]
    pub function: Option<String>,
    #[serde(default)]
    pub router_options: Option<Vec<RouterOption>>,
    #[serde(default)]
    pub input_identifier: Option<String>,
    #[serde(default)]
    pub input_type: Option<String>,
    // Additional fields based on screen type
}

table! {
    screens (id) {
        id -> Integer,
        text -> Text,
        screen_type -> Text,
        default_next_screen -> Text,
        service_code -> Nullable<Text>,
        menu_items -> Nullable<Integer>,
        function -> Nullable<Integer>,
        router_options -> Nullable<Integer>,
        input_identifier -> Nullable<Text>,
        input_type -> Nullable<Text>,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
pub struct MenuItem {
    pub option: String,
    pub display_name: String,
    pub next_screen: String,
}

impl FromSql<diesel::sql_types::Text, Sqlite> for MenuItem {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let s = <String as diesel::deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)?;
        let parts: Vec<&str> = s.split(',').collect();
        Ok(MenuItem {
            option: parts[0].to_string(),
            display_name: parts[1].to_string(),
            next_screen: parts[2].to_string(),
        })
    }
}

table! {
    menu_items (id) {
        id -> Integer,
        name -> Text,
        option -> Text,
        display_name -> Text,
        next_screen -> Text,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
pub struct RouterOption {
    pub router_option: String,
    pub next_screen: String,
}

table! {
    router_options (id) {
        id -> Integer,
        router_option -> Text,
        next_screen -> Text,
    }
}

// impl Database<MenuItem> for DatabaseManager {
//     fn insert(&mut self, menu_item: MenuItem, extra: Option<String>) -> Result<(), Box<dyn Error>> {
//         use self::menu_items::dsl::*;

//         let new_menu_item = MenuItem {
//             option: menu_item.option,
//             display_name: menu_item.display_name,
//             next_screen: menu_item.next_screen,
//         };

//         diesel::insert_into(menu_items)
//             .values(&new_menu_item)
//             .execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn update(&self, id: i32, menu_item: MenuItem) -> Result<(), Box<dyn Error>> {
//         use self::menu_items::dsl::*;

//         let new_menu_item = MenuItem {
//             option: menu_item.option,
//             display_name: menu_item.display_name,
//             next_screen: menu_item.next_screen,
//         };

//         diesel::update(menu_items.find(id))
//             .set(new_menu_item)
//             .execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
//         use self::menu_items::dsl::*;

//         diesel::delete(menu_items.find(id)).execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn get(&self, id: i32) -> Result<MenuItem, Box<dyn Error>> {
//         use self::menu_items::dsl::*;

//         let menu_item = menu_items
//             .find(id)
//             .first::<MenuItem>(&mut self.connection)?;

//         Ok(menu_item)
//     }

//     fn get_all(&self) -> Result<Vec<MenuItem>, Box<dyn Error>> {
//         use self::menu_items::dsl::*;

//         let results = menu_items.load::<MenuItem>(&mut self.connection)?;

//         Ok(results)
//     }
// }

// impl Database<RouterOption> for DatabaseManager {
//     fn insert(&mut self, routers: RouterOption) -> Result<(), Box<dyn Error>> {
//         use self::router_options::dsl::*;

//         let new_router_option = RouterOption {
//             router_option: routers.router_option,
//             next_screen: routers.next_screen,
//         };

//         diesel::insert_into(router_options)
//             .values(&new_router_option)
//             .execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn update(&self, id: i32, routers: RouterOption) -> Result<(), Box<dyn Error>> {
//         use self::router_options::dsl::*;

//         let new_router_option = RouterOption {
//             router_option: routers.router_option,
//             next_screen: routers.next_screen,
//         };

//         diesel::update(router_options.find(id))
//             .set(new_router_option)
//             .execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
//         use self::router_options::dsl::*;

//         diesel::delete(router_options.find(id)).execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn get(&self, id: i32) -> Result<RouterOption, Box<dyn Error>> {
//         use self::router_options::dsl::*;

//         let router = router_options
//             .find(id)
//             .first::<RouterOption>(&mut self.connection)?;

//         Ok(router)
//     }

//     fn get_all(&self) -> Result<Vec<RouterOption>, Box<dyn Error>> {
//         use self::router_options::dsl::*;

//         let results = router_options.load::<RouterOption>(&mut self.connection)?;

//         Ok(results)
//     }
// }

// impl Database<Screen> for DatabaseManager {
//     fn insert(&mut self, screen: Screen) -> Result<(), Box<dyn Error>> {
//         use self::screens::dsl::*;

//         // insert the menu items and router options first

//         if let Some(menu_itemz) = screen.menu_items {
//             for (_, menu_item) in menu_items.iter() {
//                 self.insert(menu_item.clone())?;
//             }
//         }

//         let new_screen = Screen {
//             text: screen.text,
//             screen_type: screen.screen_type,
//             default_next_screen: screen.default_next_screen,
//             service_code: screen.service_code,
//             menu_items: None,
//             function: screen.function,
//             router_options: None,
//             input_identifier: screen.input_identifier,
//             input_type: screen.input_type,
//         };

//         diesel::insert_into(screens)
//             .values(&new_screen)
//             .execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn update(&self, id: i32, screen: Screen) -> Result<(), Box<dyn Error>> {
//         use self::screens::dsl::*;

//         let new_screen = Screen {
//             text: screen.text,
//             screen_type: screen.screen_type,
//             default_next_screen: screen.default_next_screen,
//             service_code: screen.service_code,
//             menu_items: screen.menu_items,
//             function: screen.function,
//             router_options: screen.router_options,
//             input_identifier: screen.input_identifier,
//             input_type: screen.input_type,
//         };

//         diesel::update(screens.find(id))
//             .set(new_screen)
//             .execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
//         use self::screens::dsl::*;

//         diesel::delete(screens.find(id)).execute(&mut self.connection)?;

//         Ok(())
//     }

//     fn get(&self, id: i32) -> Result<Screen, Box<dyn Error>> {
//         use self::screens::dsl::*;

//         let screen = screens
//             .find(id)
//             .first::<Screen>(&mut self.connection)?;

//         Ok(screen)
//     }

//     fn get_all(&self) -> Result<Vec<Screen>, Box<dyn Error>> {
//         use self::screens::dsl::*;

//         let results = screens.load::<Screen>(&mut self.connection)?;

//         Ok(results)
//     }
// }