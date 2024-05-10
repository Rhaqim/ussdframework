use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::error::Error;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "menu.sqlite3";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let manager = ConnectionManager::<SqliteConnection>::new("menu.sqlite3");
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub trait Database<T> {
    fn create(&mut self, model: T) -> Result<(), Box<dyn Error>>;
    fn update(&mut self, id: i32, model: T) -> Result<(), Box<dyn Error>>;
    fn delete(&mut self, id: i32) -> Result<(), Box<dyn Error>>;
    fn get_by_id(&mut self, id: i32) -> Result<T, Box<dyn Error>>;
    fn get_many(&mut self) -> Result<Vec<T>, Box<dyn Error>>;
    fn get_by_query(&mut self, query: String) -> Result<Vec<T>, Box<dyn Error>>;
}

pub struct DatabaseManager {
    pub connection: SqliteConnection,
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseManager {
    pub fn new() -> DatabaseManager {
        let connection = establish_connection();
        let pool = establish_pool();
        DatabaseManager { connection, pool }
    }
}
