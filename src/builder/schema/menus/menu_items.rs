use std::error::Error;

use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::sqlite::{Sqlite, SqliteValue};

use diesel::deserialize::FromSql;

use serde::ser::StdError;
use serde::{Deserialize, Serialize};

use crate::builder::{Database, DatabaseManager};

// Define structure for a menu item
#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
pub struct MenuItem {
    pub screen_id: i32,
    pub name: String,
    pub option: String,
    pub display_name: String,
    pub next_screen: String,
}

impl FromSql<Text, Sqlite> for MenuItem {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let s = <String as diesel::deserialize::FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let parts: Vec<&str> = s.split(',').collect();
        Ok(MenuItem {
            screen_id: parts[0].parse().unwrap(),
            name: parts[1].to_string(),
            option: parts[2].to_string(),
            display_name: parts[3].to_string(),
            next_screen: parts[4].to_string(),
        })
    }
}

table! {
    menu_items (id) {
        id -> Integer,
        screen_id -> Integer,
        name -> Text,
        option -> Text,
        display_name -> Text,
        next_screen -> Text,
    }
}

impl
    diesel::Queryable<
        (
            diesel::sql_types::Integer,
            diesel::sql_types::Integer,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
        ),
        Sqlite,
    > for MenuItem
{
    type Row = (i32, i32, String, String, String, String);

    fn build(row: Self::Row) -> Result<MenuItem, Box<(dyn StdError + Send + Sync + 'static)>> {
        Ok(MenuItem {
            screen_id: row.1,
            name: row.2,
            option: row.3,
            display_name: row.4,
            next_screen: row.5,
        })
    }
}

impl Database<MenuItem> for DatabaseManager {
    fn create(&mut self, model: MenuItem) -> Result<(), Box<dyn Error>> {
        diesel::insert_into(menu_items::table)
            .values(&model)
            .execute(&mut self.connection)?;
        Ok(())
    }

    fn update(&mut self, id: i32, model: MenuItem) -> Result<(), Box<dyn Error>> {
        diesel::update(menu_items::table.find(id))
            .set(&model)
            .execute(&mut self.connection)?;
        Ok(())
    }

    fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        diesel::delete(menu_items::table.find(id)).execute(&mut self.connection)?;
        Ok(())
    }

    fn get(&mut self, id: i32) -> Result<MenuItem, Box<dyn Error>> {
        let result = menu_items::table.find(id).first(&mut self.connection)?;
        Ok(result)
    }

    fn get_many(&mut self) -> Result<Vec<MenuItem>, Box<dyn Error>> {
        let result = menu_items::table.load::<MenuItem>(&mut self.connection)?;
        Ok(result)
    }
}
