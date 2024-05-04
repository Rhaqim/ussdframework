use std::{collections::HashMap, error::Error};

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use diesel::{deserialize::FromSql, serialize::ToSql, sql_types::Text, sqlite::SqliteValue};

use serde::ser::StdError;
use serde::{Deserialize, Serialize};

use crate::builder::{Database, DatabaseManager};

// #[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
// pub struct MenuItem {
//     pub option: String,
//     pub display_name: String,
//     pub next_screen: String,
// }

// impl FromSql<diesel::sql_types::Text, Sqlite> for MenuItem {
//     fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
//         let s =
//             <String as diesel::deserialize::FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(
//                 bytes,
//             )?;
//         let parts: Vec<&str> = s.split(',').collect();
//         Ok(MenuItem {
//             option: parts[0].to_string(),
//             display_name: parts[1].to_string(),
//             next_screen: parts[2].to_string(),
//         })
//     }
// }

// table! {
//     menu_items (id) {
//         id -> Integer,
//         option -> Text,
//         display_name -> Text,
//         next_screen -> Text,
//     }
// }

// #[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
// pub struct RouterOption {
//     pub router_option: String,
//     pub next_screen: String,
// }

// table! {
//     router_options (id) {
//         id -> Integer,
//         router_option -> Text,
//         next_screen -> Text,
//     }
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub enum ScreenType {
//     Initial,
//     Menu,
//     Input,
//     Function,
//     Router,
//     Quit,
// }

// // Define structure for a screen
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Screen {
//     pub text: String,
//     pub screen_type: ScreenType,
//     pub default_next_screen: String,
//     #[serde(default)]
//     pub service_code: Option<String>,
//     #[serde(default)]
//     pub menu_items: Option<HashMap<String, MenuItem>>,
//     #[serde(default)]
//     pub function: Option<String>,
//     #[serde(default)]
//     pub router_options: Option<Vec<RouterOption>>,
//     #[serde(default)]
//     pub input_identifier: Option<String>,
//     #[serde(default)]
//     pub input_type: Option<String>,
//     // Additional fields based on screen type
// }

// table! {
//     screens (id) {
//         id -> Integer,
//         text -> Text,
//         screen_type -> Text,
//         default_next_screen -> Text,
//         service_code -> Nullable<Text>,
//         menu_items -> Nullable<Integer>,
//         function -> Nullable<Integer>,
//         router_options -> Nullable<Integer>,
//         input_identifier -> Nullable<Text>,
//         input_type -> Nullable<Text>,
//     }
// }

// use diesel::{self, prelude::*};
// use diesel::sqlite::SqliteConnection;
// use diesel::deserialize::FromSql;
// use diesel::sql_types::Text;
// use std::collections::HashMap;
// use std::error::Error;

// Define structure for a menu item
#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
pub struct MenuItem {
    pub option: String,
    pub display_name: String,
    pub next_screen: String,
}

impl FromSql<Text, Sqlite> for MenuItem {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let s = <String as diesel::deserialize::FromSql<Text, Sqlite>>::from_sql(bytes)?;
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
        option -> Text,
        display_name -> Text,
        next_screen -> Text,
    }
}

// Define structure for a router option
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

// Define enum for screen types
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum ScreenType {
    Initial,
    Menu,
    Input,
    Function,
    Router,
    Quit,
}

impl ToSql<Text, Sqlite> for ScreenType {
    fn to_sql(&self, out: &mut diesel::serialize::Output<Sqlite>) -> diesel::serialize::Result {
        let s = match self {
            ScreenType::Initial => "Initial",
            ScreenType::Menu => "Menu",
            ScreenType::Input => "Input",
            ScreenType::Function => "Function",
            ScreenType::Router => "Router",
            ScreenType::Quit => "Quit",
        };

        ToSql::<Text, Sqlite>::to_sql(s, out)
    }
}

impl FromSql<Text, Sqlite> for ScreenType {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let s = <String as diesel::deserialize::FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match s.as_str() {
            "Initial" => Ok(ScreenType::Initial),
            "Menu" => Ok(ScreenType::Menu),
            "Input" => Ok(ScreenType::Input),
            "Function" => Ok(ScreenType::Function),
            "Router" => Ok(ScreenType::Router),
            "Quit" => Ok(ScreenType::Quit),
            _ => Err("Invalid value for ScreenType".into()),
        }
    }
}

impl diesel::Queryable<Text, Sqlite> for ScreenType {
    type Row = String;

    fn build(row: Self::Row) -> Result<ScreenType, Box<(dyn StdError + Send + Sync + 'static)>> {
        match row.as_str() {
            "Initial" => Ok(ScreenType::Initial),
            "Menu" => Ok(ScreenType::Menu),
            "Input" => Ok(ScreenType::Input),
            "Function" => Ok(ScreenType::Function),
            "Router" => Ok(ScreenType::Router),
            "Quit" => Ok(ScreenType::Quit),
            _ => Err("Invalid value for ScreenType".into()),
        }
    }
}

// Define structure for a screen
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
pub struct Screen {
    pub text: String,
    pub screen_type: ScreenType,
    pub default_next_screen: String,
    pub service_code: Option<String>,
    pub menu_items: Option<HashMap<String, MenuItem>>,
    pub function: Option<String>,
    pub router_options: Option<Vec<RouterOption>>,
    pub input_identifier: Option<String>,
    pub input_type: Option<String>,
}

table! {
    screens (id) {
        id -> Integer,
        text -> Text,
        screen_type -> Text,
        default_next_screen -> Text,
        service_code -> Nullable<Text>,
        menu_items -> Nullable<Text>, // Changed from Integer to Text
        function -> Nullable<Text>, // Changed from Integer to Text
        router_options -> Nullable<Text>, // Changed from Integer to Text
        input_identifier -> Nullable<Text>,
        input_type -> Nullable<Text>,
    }
}

impl Database<Screen> for DatabaseManager {
    fn create(&mut self, model: Screen) -> Result<(), Box<dyn Error>> {
        diesel::insert_into(screens::table)
            .values(&model)
            .execute(&mut self.connection)?;
        Ok(())
    }

    fn update(&mut self, id: i32, model: Screen) -> Result<(), Box<dyn Error>> {
        diesel::update(screens::table.find(id))
            .set(&model)
            .execute(&mut self.connection)?;
        Ok(())
    }

    fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        diesel::delete(screens::table.find(id)).execute(&mut self.connection)?;
        Ok(())
    }

    fn get(&mut self, id: i32) -> Result<Screen, Box<dyn Error>> {
        let result = screens::table.find(id).first(&mut self.connection)?;
        Ok(result)
    }

    fn get_many(&mut self) -> Result<Vec<Screen>, Box<dyn Error>> {
        let result = screens::table.load::<Screen>(&mut self.connection)?;
        Ok(result)
    }
}
