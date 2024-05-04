use std::error::Error;

use diesel::prelude::*;
use serde::ser::StdError;
use serde::{Deserialize, Serialize};

use crate::builder::{Database, DatabaseManager};
use crate::core::USSDService;

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, QueryableByName, AsChangeset)]
pub struct Service {
    pub name: String,
    pub function_name: String,
    pub function_url: Option<String>,
    pub data_key: String,
    pub service_code: Option<String>,
}

table! {
    services (id) {
        id -> Integer,
        name -> Text,
        function_name -> Text,
        function_url -> Nullable<Text>,
        data_key -> Text,
        service_code -> Nullable<Text>,
    }
}

impl Service {
    pub fn from(name: &str, service: USSDService) -> Self {
        Service {
            name: name.to_string(),
            function_name: service.function_name,
            function_url: service.function_url,
            data_key: service.data_key,
            service_code: service.service_code,
        }
    }

    pub fn to_ussd_service(&self) -> USSDService {
        USSDService {
            function_name: self.function_name.clone(),
            function_url: self.function_url.clone(),
            data_key: self.data_key.clone(),
            service_code: self.service_code.clone(),
        }
    }
}

impl
    diesel::Queryable<
        (
            diesel::sql_types::Integer,
            diesel::sql_types::Text,
            diesel::sql_types::Text,
            diesel::sql_types::Nullable<diesel::sql_types::Text>,
            diesel::sql_types::Text,
            diesel::sql_types::Nullable<diesel::sql_types::Text>,

        ),
        diesel::sqlite::Sqlite,
    > for Service
{
    type Row = (i32, String, String, Option<String>, String, Option<String>);

    fn build(row: Self::Row) -> Result<Service, Box<(dyn StdError + Send + Sync + 'static)>> {
        Ok(Service {
            name: row.1,
            function_name: row.2,
            function_url: row.3,
            data_key: row.4,
            service_code: row.5,
        })
    }
}

impl Database<Service> for DatabaseManager {
    fn create(&mut self, model: Service) -> Result<(), Box<dyn Error>> {
        diesel::insert_into(services::table)
            .values(&model)
            .execute(&mut self.connection)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
            .map(|_| ())
    }

    fn update(&mut self, id: i32, model: Service) -> Result<(), Box<dyn Error>> {
        diesel::update(services::table.find(id))
            .set(&model)
            .execute(&mut self.connection)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
            .map(|_| ())
    }

    fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        diesel::delete(services::table.find(id))
            .execute(&mut self.connection)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
            .map(|_| ())
    }

    fn get(&mut self, id: i32) -> Result<Service, Box<dyn Error>> {
        services::table
            .find(id)
            .first(&mut self.connection)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn get_many(&mut self) -> Result<Vec<Service>, Box<dyn Error>> {
        services::table
            .load(&mut self.connection)
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
