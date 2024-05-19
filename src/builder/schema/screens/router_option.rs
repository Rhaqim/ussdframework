use std::error::Error;

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use serde::ser::StdError;
use serde::{Deserialize, Serialize};

use crate::builder::{Database, DatabaseManager, QueryEnum};

// Define structure for a router option
#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable, AsChangeset)]
pub struct RouterOption {
    pub screen_name: String,
    pub router_option: String,
    pub next_screen: String,
}

table! {
    router_options (id) {
        id -> Integer,
        screen_name -> Text,
        router_option -> Text,
        next_screen -> Text,
    }
}

impl RouterOption {
    pub fn to_ussd_router_option(&self) -> crate::core::ussd_screens::RouterOption {
        crate::core::ussd_screens::RouterOption {
            router_option: self.router_option.clone(),
            next_screen: self.next_screen.clone(),
        }
    }

    pub fn from_ussd_router_option(
        screen_name: String,
        router_option: crate::core::ussd_screens::RouterOption,
    ) -> RouterOption {
        RouterOption {
            screen_name,
            router_option: router_option.router_option,
            next_screen: router_option.next_screen,
        }
    }
}

impl
    diesel::Queryable<
        (
            diesel::sql_types::Integer,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
        ),
        Sqlite,
    > for RouterOption
{
    type Row = (i32, String, String, String);

    fn build(row: Self::Row) -> Result<RouterOption, Box<(dyn StdError + Send + Sync + 'static)>> {
        Ok(RouterOption {
            screen_name: row.1,
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

    fn get_by_id(&mut self, id: i32) -> Result<RouterOption, Box<dyn Error>> {
        let result = router_options::table.find(id).first(&mut self.connection)?;
        Ok(result)
    }

    fn get_many(&mut self) -> Result<Vec<RouterOption>, Box<dyn Error>> {
        let result = router_options::table.load::<RouterOption>(&mut self.connection)?;
        Ok(result)
    }

    // fn get_by_query(&mut self, query: String) -> Result<Vec<RouterOption>, Box<dyn Error>> {
    //     let result = router_options::table
    //         .filter(router_options::screen_name.eq(query))
    //         .load::<RouterOption>(&mut self.connection)?;
    //     Ok(result)
    // }

    fn get_by_query_enum(&mut self, query: QueryEnum) -> Result<Vec<RouterOption>, Box<dyn Error>> {
        use self::router_options::dsl::*;

        let result = match query {
            QueryEnum::ID(q_id) => router_options
                .filter(id.eq(q_id))
                .load::<RouterOption>(&mut self.connection)?,
            QueryEnum::ScreenName(q_screen_name) => router_options
                .filter(screen_name.eq(q_screen_name))
                .load::<RouterOption>(&mut self.connection)?,
            _ => Vec::new(),
        };

        Ok(result)
    }
}
