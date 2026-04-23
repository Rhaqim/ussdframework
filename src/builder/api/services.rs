use serde::ser::StdError;

use actix_web::{web, HttpResponse, Responder};

use crate::builder::{Database, DatabaseManager, QueryEnum};

use crate::builder::ServiceModel;

use super::{with_database, PathInfo, ServiceModelUpdate};

// Create operation
pub async fn create(service: web::Json<ServiceModel>) -> impl Responder {
    with_database(move |_manager| {
        // Insert the screen into the database
        let service = service.into_inner();

        let result = _manager.create(service.clone());

        async move {
            match result {
                Ok(_) => HttpResponse::Ok().body("Screen created successfully"),
                Err(_) => HttpResponse::InternalServerError().body("Error creating screen"),
            }
        }
    })
    .await
}

// Update operation
pub async fn update(service: web::Json<ServiceModelUpdate>) -> impl Responder {
    with_database(move |_manager| {
        // Update the screen in the database
        let service = service.into_inner();

        let result = _manager.update(service.id, service.service.clone());

        async move {
            match result {
                Ok(_) => HttpResponse::Ok().body("Screen updated successfully"),
                Err(_) => HttpResponse::InternalServerError().body("Error updating screen"),
            }
        }
    })
    .await
}

// Delete operation
pub async fn delete(path: web::Path<PathInfo>) -> impl Responder {
    with_database(move |_manager| {
        // Delete the screen from the database

        let _id = path.into_inner().id.unwrap_or_default();

        let result = <DatabaseManager as Database<ServiceModel>>::delete(_manager, _id);

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
            Some(id) => <DatabaseManager as Database<ServiceModel>>::get_by_id(_manager, id),
            None => match path.name {
                Some(name) => {
                    <DatabaseManager as Database<ServiceModel>>::get_by_name(_manager, name)
                }
                None => Err(Box::<dyn StdError>::from("No id or name provided")),
            },
        };

        async move {
            match result {
                Ok(screen) => HttpResponse::Ok().json(screen),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error getting service: {e}")),
            }
        }
    })
    .await
}

// Get multiple operation
pub async fn get_multiple(query: web::Query<super::MultipleQuery>) -> impl Responder {
    with_database(move |_manager| {
        let q = query.into_inner();
        let query_enum = if let Some(screen_name) = q.screen_name {
            QueryEnum::ScreenName(screen_name)
        } else if let Some(name) = q.name {
            QueryEnum::Name(name)
        } else if let Some(id) = q.id {
            QueryEnum::ID(id)
        } else if let Some(service_code) = q.service_code {
            QueryEnum::ServiceCode(service_code)
        } else {
            return futures_util::future::Either::Right(async {
                HttpResponse::BadRequest().body("No query parameter provided")
            });
        };

        let result =
            <DatabaseManager as Database<ServiceModel>>::get_by_query_enum(_manager, query_enum);

        futures_util::future::Either::Left(async move {
            match result {
                Ok(screens) => HttpResponse::Ok().json(screens),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error getting services: {e}")),
            }
        })
    })
    .await
}

// Get all operation
pub async fn get_all() -> impl Responder {
    with_database(move |_manager| {
        // Get all screens from the database

        let result = <DatabaseManager as Database<ServiceModel>>::get_many(_manager);

        async move {
            match result {
                Ok(screens) => HttpResponse::Ok().json(screens),
                Err(_) => HttpResponse::InternalServerError().body("Error getting screens"),
            }
        }
    })
    .await
}
