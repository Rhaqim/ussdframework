use std::error::Error;

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use serde::ser::StdError;
use serde::{Deserialize, Serialize};

use crate::builder::{Database, DatabaseManager};

// Define structure for a router option
#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
pub struct RouterOption {
    pub screen_id: i32,
    pub router_option: String,
    pub next_screen: String,
}

table! {
    router_options (id) {
        id -> Integer,
        screen_id -> Integer,
        router_option -> Text,
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
        ),
        Sqlite,
    > for RouterOption
{
    type Row = (i32, i32, String, String);

    fn build(row: Self::Row) -> Result<RouterOption, Box<(dyn StdError + Send + Sync + 'static)>> {
        Ok(RouterOption {
            screen_id: row.1,
            router_option: row.2,
            next_screen: row.3,
        })
    }
}

impl Database<RouterOption> for DatabaseManager {
    fn create(&mut self, model: RouterOption) -> Result<(), Box<dyn Error>> {
        diesel::insert_into(router_options::table)
            .values(&model)
            .execute(&mut self.connection)?;
        Ok(())
    }

    fn update(&mut self, id: i32, model: RouterOption) -> Result<(), Box<dyn Error>> {
        diesel::update(router_options::table.find(id))
            .set(&model)
            .execute(&mut self.connection)?;
        Ok(())
    }

    fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        diesel::delete(router_options::table.find(id)).execute(&mut self.connection)?;
        Ok(())
    }

    fn get(&mut self, id: i32) -> Result<RouterOption, Box<dyn Error>> {
        let result = router_options::table.find(id).first(&mut self.connection)?;
        Ok(result)
    }

    fn get_many(&mut self) -> Result<Vec<RouterOption>, Box<dyn Error>> {
        let result = router_options::table.load::<RouterOption>(&mut self.connection)?;
        Ok(result)
    }
}
