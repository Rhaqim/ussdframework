use diesel::{sqlite::SqliteConnection, Connection};
use std::error::Error;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "database.db";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub trait Database<T> {
    fn insert(&mut self, model: T, extra: Option<String>) -> Result<(), Box<dyn Error>>;
    fn update(&self, id: i32, model: T) -> Result<(), Box<dyn Error>>;
    fn delete(&self, id: i32) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: i32) -> Result<T, Box<dyn Error>>;
    fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>>;
}

pub struct DatabaseManager {
    pub connection: SqliteConnection,
}

impl DatabaseManager {
    pub fn new() -> DatabaseManager {
        let connection = establish_connection();
        DatabaseManager { connection }
    }
}
