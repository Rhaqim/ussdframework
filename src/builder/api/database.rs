use actix_web::{HttpResponse, Responder};

use crate::builder::{Database, DatabaseManager, Screen, Service};

pub async fn database_start() -> impl Responder {
    let mut db = DatabaseManager::new();
    
    HttpResponse::Ok().body("Database started")
}