use std::error::Error;

use diesel::deserialize::FromSqlRow;
use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::sqlite::{Sqlite, SqliteValue};

use serde::ser::StdError;
use serde::{Deserialize, Serialize};

use crate::builder::{Database, DatabaseManager};
use crate::core::ussd_screens::USSDScreen;
use crate::core::ScreenType;

// Define structure for a screen
#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset, FromSqlRow)]
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
        text -> Text,
        screen_type -> Text,
        default_next_screen -> Text,
        service_code -> Nullable<Text>,
        function -> Nullable<Text>,
        input_identifier -> Nullable<Text>,
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

impl diesel::deserialize::FromSqlRow<diesel::sql_types::Untyped, diesel::sqlite::Sqlite> for Screen {
    fn build_from_row<'a>(row: &impl diesel::row::Row<'a, diesel::sqlite::Sqlite>) -> Result<Screen, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Screen {
            name: row.get(1)?,
            text: row.get(2)?,
            screen_type: row.get(3)?,
            default_next_screen: row.get(4)?,
            service_code: row.get(5)?,
            function: row.get(6)?,
            input_identifier: row.get(7)?,
            input_type: row.get(8)?,
        })
    }
}

impl diesel::deserialize::FromSql<Text, Sqlite> for Screen {
    fn from_sql(
        bytes: SqliteValue<'_, '_, '_>,
    ) -> Result<Self, Box<dyn StdError + Send + Sync + 'static>> {
        let s = <String as diesel::deserialize::FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let parts: Vec<&str> = s.split(',').collect();
        Ok(Screen {
            name: parts[0].to_string(),
            text: parts[1].to_string(),
            screen_type: parts[2].to_string(),
            default_next_screen: parts[3].to_string(),
            service_code: Some(parts[4].to_string()),
            function: Some(parts[5].to_string()),
            input_identifier: Some(parts[6].to_string()),
            input_type: Some(parts[7].to_string()),
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

    fn get_by_id(&mut self, id: i32) -> Result<Screen, Box<dyn Error>> {
        let result = screens::table.find(id).first(&mut self.connection)?;
        Ok(result)
    }

    fn get_many(&mut self) -> Result<Vec<Screen>, Box<dyn Error>> {
        let result = screens::table.load::<Screen>(&mut self.connection)?;
        Ok(result)
    }

    fn get_by_query(&mut self, query: String) -> Result<Vec<Screen>, Box<dyn Error>> {
        diesel::dsl::sql_query(query)
            .load(&mut self.connection)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
