use std::{collections::HashMap, error::Error};

use diesel::{deserialize::FromSql, prelude::*, sqlite::SqliteValue};
use serde::{Deserialize, Serialize};

use diesel::sqlite::Sqlite;

use crate::builder::{Database, DatabaseManager};

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

