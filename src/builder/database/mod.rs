pub mod migrations;

pub use migrations::run_migration;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Backend-specific connection type alias.
// db-postgres takes precedence if both features are enabled.
#[cfg(all(feature = "db-sqlite", not(feature = "db-postgres")))]  
pub type DbConnection = diesel::sqlite::SqliteConnection;

#[cfg(feature = "db-postgres")]
pub type DbConnection = diesel::pg::PgConnection;

#[cfg(all(feature = "db-sqlite", not(feature = "db-postgres")))]
fn default_db_url() -> String {
    "menu.sqlite3".to_string()
}

#[cfg(feature = "db-postgres")]
fn default_db_url() -> String {
    "postgres://localhost/ussd_menu".to_string()
}

pub fn establish_connection() -> DbConnection {
    let database_url = std::env::var("USSD_DATABASE_URL")
        .unwrap_or_else(|_| default_db_url());
    DbConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub trait Database<T> {
    fn create(&mut self, model: T) -> Result<(), Box<dyn Error>>;
    fn update(&mut self, id: i32, model: T) -> Result<(), Box<dyn Error>>;
    fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>>;
    fn get_by_id(&mut self, id: i32) -> Result<T, Box<dyn Error>>;
    fn get_by_name(&mut self, name: String) -> Result<T, Box<dyn Error>>;
    fn get_many(&mut self) -> Result<Vec<T>, Box<dyn Error>>;
    // fn get_by_query(&mut self, query: String) -> Result<Vec<T>, Box<dyn Error>>;
    fn get_by_query_enum(&mut self, query: QueryEnum) -> Result<Vec<T>, Box<dyn Error>>;
}

pub struct DatabaseManager {
    pub connection: DbConnection,
}

impl DatabaseManager {
    pub fn new() -> DatabaseManager {
        let connection = establish_connection();
        DatabaseManager { connection }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum QueryEnum {
    ID(i32),
    Name(String),
    ScreenName(String),
    ScreenType(String),
    ServiceCode(String),
    Function(String),
    DataKey(String),
}

impl Default for QueryEnum {
    fn default() -> Self {
        QueryEnum::ID(0)
    }
}
