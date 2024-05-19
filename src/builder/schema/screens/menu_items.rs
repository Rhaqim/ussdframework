use std::error::Error;

use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::sqlite::{Sqlite, SqliteValue};

use diesel::deserialize::FromSql;

use serde::ser::StdError;
use serde::{Deserialize, Serialize};

use crate::builder::{Database, DatabaseManager, QueryEnum};
use crate::core::ussd_screens::USSDMenuItems;

// Define structure for a menu item
#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
pub struct MenuItem {
    pub screen_name: String,
    pub name: String,
    pub option: String,
    pub display_name: String,
    pub next_screen: String,
}

impl MenuItem {
    pub fn to_ussd_menu_item(&self) -> (String, USSDMenuItems) {
        let menu_item = USSDMenuItems {
            option: self.option.clone(),
            display_name: self.display_name.clone(),
            next_screen: self.next_screen.clone(),
        };

        (self.name.clone(), menu_item)
    }

    pub fn from_ussd_menu_item(
        screen_name: String,
        name: String,
        menu_item: USSDMenuItems,
    ) -> MenuItem {
        MenuItem {
            screen_name,
            name,
            option: menu_item.option,
            display_name: menu_item.display_name,
            next_screen: menu_item.next_screen,
        }
    }
}

impl FromSql<Text, Sqlite> for MenuItem {
    fn from_sql(bytes: SqliteValue<'_, '_, '_>) -> diesel::deserialize::Result<Self> {
        let s = <String as diesel::deserialize::FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let parts: Vec<&str> = s.split(',').collect();
        Ok(MenuItem {
            screen_name: parts[0].to_string(),
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
        screen_name -> Text,
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
            diesel::sql_types::Text,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
        ),
        Sqlite,
    > for MenuItem
{
    type Row = (i32, String, String, String, String, String);

    fn build(row: Self::Row) -> Result<MenuItem, Box<(dyn StdError + Send + Sync + 'static)>> {
        Ok(MenuItem {
            screen_name: row.1,
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

    fn get_by_id(&mut self, id: i32) -> Result<MenuItem, Box<dyn Error>> {
        let result = menu_items::table.find(id).first(&mut self.connection)?;
        Ok(result)
    }

    fn get_many(&mut self) -> Result<Vec<MenuItem>, Box<dyn Error>> {
        let result = menu_items::table.load::<MenuItem>(&mut self.connection)?;
        Ok(result)
    }

    // fn get_by_query(&mut self, query: String) -> Result<Vec<MenuItem>, Box<dyn Error>> {
    //     let result = menu_items::table
    //         .filter(menu_items::screen_name.eq(query))
    //         .load::<MenuItem>(&mut self.connection)?;
    //     Ok(result)
    // }

    fn get_by_query_enum(&mut self, query: QueryEnum) -> Result<Vec<MenuItem>, Box<dyn Error>> {
        use self::menu_items::dsl::*;

        let result = match query {
            QueryEnum::ID(q_id) => menu_items
                .filter(id.eq(q_id))
                .load::<MenuItem>(&mut self.connection)?,
            QueryEnum::ScreenName(q_screen_name) => menu_items
                .filter(screen_name.eq(q_screen_name))
                .load::<MenuItem>(&mut self.connection)?,
            QueryEnum::Name(q_name) => menu_items
                .filter(name.like(format!("%{}%", q_name)))
                .load::<MenuItem>(&mut self.connection)?,
            _ => Vec::new(),
        };

        Ok(result)
    }
}
