use std::sync::RwLock;

use actix_web::{web, HttpResponse, Responder};

use crate::builder::{Database, DatabaseManager, Screen};

// Create operation
pub async fn create(db_manager: web::Data<DatabaseManager>) -> impl Responder {
    // Implement create logic here
    HttpResponse::Ok().body("Create operation executed")
}

// Update operation
pub async fn update(db_manager: web::Data<DatabaseManager>) -> impl Responder {
    // Implement update logic here
    HttpResponse::Ok().body("Update operation executed")
}

// Delete operation
pub async fn delete(db_manager: web::Data<DatabaseManager>) -> impl Responder {
    // Implement delete logic here
    HttpResponse::Ok().body("Delete operation executed")
}

// Get operation
pub async fn get(db_manager: web::Data<DatabaseManager>) -> impl Responder {
    // Implement get logic here
    HttpResponse::Ok().body("Get operation executed")
}

// Get multiple operation
pub async fn get_multiple(db_manager: web::Data<DatabaseManager>) -> impl Responder {
    // Implement get multiple logic here
    HttpResponse::Ok().body("Get multiple operation executed")
}

// Get all operation
pub async fn get_all(db_manager: web::Data<DatabaseManager>) -> impl Responder {
    // Implement get all logic here
    HttpResponse::Ok().body("Get all operation executed")
}

pub async fn get_list(db_manager: web::Data<DatabaseManager>) -> impl Responder {
    let mut manager = DatabaseManager::new();

    let screens: Vec<Screen> = manager
        .get_by_query("SELECT * FROM screens".to_string())
        .unwrap();

    HttpResponse::Ok().json(screens)
}
