use std::sync::RwLock;

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::builder::{Database, DatabaseManager};

use crate::builder::schema::services::model_service::Service;

// Create operation
pub async fn create(
    service: web::Json<Service>,
) -> impl Responder {let service = service.into_inner();

    let mut manager = DatabaseManager::new();

    manager.create(service).unwrap();

    HttpResponse::Ok().body("Create operation executed")
}

// Update operation
pub async fn update(
    
    service: web::Json<Service>,
) -> impl Responder {
    // Implement update logic here
    HttpResponse::Ok().body("Update operation executed")
}

// Delete operation
pub async fn delete() -> impl Responder {
    // Implement delete logic here
    HttpResponse::Ok().body("Delete operation executed")
}

// Get operation
pub async fn get() -> impl Responder {
    // Implement get logic here
    let response = json!(
        [{
            "id": 1,
            "name": "Service 1",
        }]
    );

    HttpResponse::Ok().json(response)
}

// Get multiple operation
pub async fn get_multiple() -> impl Responder {
    // Implement get multiple logic here
    HttpResponse::Ok().body("Get multiple operation executed")
}

// Get all operation
pub async fn get_all() -> impl Responder {
    // Implement get all logic here
    HttpResponse::Ok().body("Get all operation executed")
}

pub async fn get_list() -> impl Responder {
    let mut manager = DatabaseManager::new();

    let services: Vec<Service> = manager.get_by_query("SELECT id, name FROM services".to_string()).unwrap();

    HttpResponse::Ok().json(services)
}