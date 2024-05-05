use actix_web::{HttpResponse, Responder};

use crate::builder::DatabaseManager;

pub async fn database_start() -> impl Responder {
    let mut _db = DatabaseManager::new();

    HttpResponse::Ok().body("Database started")
}
