use std::error::Error;

use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

use serde::ser::StdError;
use serde::{Deserialize, Serialize};

use crate::builder::{Database, DatabaseManager};
use crate::core::ussd_screens::USSDScreen;
use crate::core::ScreenType;

// Define structure for a screen
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
pub struct Screen {
    pub name: String,
    pub text: String,
    pub screen_type: String,
    pub default_next_screen: String,
    pub service_code: Option<String>,
    pub function: Option<String>,
    pub input_identifier: Option<String>,
    pub input_type: Option<String>,
}

impl Screen {
    pub fn to_ussd_screen(&self) -> USSDScreen {
        USSDScreen {
            text: self.text.clone(),
            screen_type: ScreenType::from_string(&self.screen_type),
            default_next_screen: self.default_next_screen.clone(),
            service_code: self.service_code.clone(),
            function: self.function.clone(),
            input_identifier: self.input_identifier.clone(),
            input_type: self.input_type.clone(),
            router_options: None,
            menu_items: None,
        }
    }
}

table! {
    screens (id) {
        id -> Integer,
        name -> Text,
        text -> Varchar,
        screen_type -> VarChar,
        default_next_screen -> Varchar,
        service_code -> Nullable<Text>,
        function -> Nullable<Text>,
        input_identifier -> Nullable<VarChar>,
        input_type -> Nullable<Text>,
    }
}

impl
    diesel::Queryable<
        (
            diesel::sql_types::Integer,
            Text,
            Text,
            Text,
            Text,
            diesel::sql_types::Nullable<Text>,
            diesel::sql_types::Nullable<Text>,
            diesel::sql_types::Nullable<Text>,
            diesel::sql_types::Nullable<Text>,
        ),
        Sqlite,
    > for Screen
{
    type Row = (
        i32,
        String,
        String,
        String,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    );

    fn build(row: Self::Row) -> Result<Screen, Box<(dyn StdError + Send + Sync + 'static)>> {
        Ok(Self {
            name: row.1,
            text: row.2,
            screen_type: row.3,
            default_next_screen: row.4,
            service_code: row.5,
            function: row.6,
            input_identifier: row.7,
            input_type: row.8,
        })
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
