use serde::ser::StdError;

use actix_web::{web, HttpResponse, Responder};

use crate::builder::{Database, DatabaseManager, QueryEnum, ScreenModel};

use super::{with_database, PathInfo, ScreeModelUpdate};

pub async fn create(screen: web::Json<ScreenModel>) -> impl Responder {
    with_database(move |_manager| {
        // Insert the screen into the database
        let screen = screen.into_inner();

        let result = _manager.create(screen.clone());

        async move {
            match result {
                Ok(_) => HttpResponse::Ok().body("Screen created successfully"),
                Err(_) => HttpResponse::InternalServerError().body("Error creating screen"),
            }
        }
    })
    .await
}

pub async fn update(screen: web::Json<ScreeModelUpdate>) -> impl Responder {
    with_database(move |_manager| {
        // Update the screen in the database
        let screen = screen.into_inner();

        let result = _manager.update(screen.id, screen.screen.clone());

        async move {
            match result {
                Ok(_) => HttpResponse::Ok().body("Screen updated successfully"),
                Err(_) => HttpResponse::InternalServerError().body("Error updating screen"),
            }
        }
    })
    .await
}

pub async fn delete(path: web::Path<PathInfo>) -> impl Responder {
    with_database(move |_manager| {
        // Delete the screen from the database

        let _id = path.into_inner().id.unwrap_or_default();

        let result = <DatabaseManager as Database<ScreenModel>>::delete(_manager, _id);

        async move {
            match result {
                Ok(_) => HttpResponse::Ok().body("Screen deleted successfully"),
                Err(_) => HttpResponse::InternalServerError().body("Error deleting screen"),
            }
        }
    })
    .await
}

// Get operation
pub async fn get(path: web::Path<PathInfo>) -> impl Responder {
    with_database(move |_manager| {
        // Get the screen from the database

        let path = path.into_inner();

        let result = match path.id {
            Some(id) => <DatabaseManager as Database<ScreenModel>>::get_by_id(_manager, id),
            None => match path.name {
                Some(name) => {
                    <DatabaseManager as Database<ScreenModel>>::get_by_name(_manager, name)
                }
                None => Err(Box::<dyn StdError>::from("No id or name provided")),
            },
        };

        async move {
            match result {
                Ok(screen) => HttpResponse::Ok().json(screen),
                Err(_) => HttpResponse::InternalServerError().body("Error getting screen"),
            }
        }
    })
    .await
}

// Get multiple operation
pub async fn get_multiple(query: web::Json<QueryEnum>) -> impl Responder {
    with_database(move |_manager| {
        // Get the screen from the database
        let query = query.into_inner();

        let result =
            <DatabaseManager as Database<ScreenModel>>::get_by_query_enum(_manager, query.clone());

        async move {
            match result {
                Ok(screens) => HttpResponse::Ok().json(screens),
                Err(_) => HttpResponse::InternalServerError().body("Error getting screens"),
            }
        }
    })
    .await
}

// Get all operation
pub async fn get_all() -> impl Responder {
    with_database(move |_manager| {
        // Get all screens from the database

        let result = <DatabaseManager as Database<ScreenModel>>::get_many(_manager);

        async move {
            match result {
                Ok(screens) => HttpResponse::Ok().json(screens),
                Err(_) => HttpResponse::InternalServerError().body("Error getting screens"),
            }
        }
    })
    .await
}
