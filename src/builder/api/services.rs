use std::sync::RwLock;

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::builder::{Database, DatabaseManager};

use crate::builder::schema::model_service::Service;

// Create operation
pub async fn create(
    db_manager: web::Data<RwLock<DatabaseManager>>,
    service: web::Json<Service>,
) -> impl Responder {
    // Implement create logic here

    let service = service.into_inner();
    let mut manager = db_manager.write().unwrap();

    manager.create(service).unwrap();

    HttpResponse::Ok().body("Create operation executed")
}

// Update operation
pub async fn update(
    db_manager: web::Data<DatabaseManager>,
    service: web::Json<Service>,
) -> impl Responder {
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
    let response = json!(
        [{
            "id": 1,
            "name": "Service 1",
        }]
    );

    HttpResponse::Ok().json(response)
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
