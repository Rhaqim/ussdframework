use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, Responder};
use futures_util::stream::StreamExt;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::builder::file::{from_json, to_json};
use crate::builder::{Database, DatabaseManager, ScreenModel, ServiceModel};
use crate::core::USSDMenu;

use super::with_database;

pub async fn process_json_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // Iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Ensure the field is a file and has a name
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap().to_string();

        // Check if the file is JSON
        if !filename.ends_with(".json") {
            return Ok(HttpResponse::BadRequest().body("Invalid file format. Must be JSON."));
        }

        // Create a temporary file to save the uploaded file
        let filepath = format!("./uploads/{}", filename);

        // Ensure the directory exists
        if let Err(e) = std::fs::create_dir_all("./uploads") {
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Failed to create directories: {}", e)));
        }

        // Clone the filepath for loading the JSON data
        let load_filepath = filepath.clone();

        let file = web::block(move || File::create(&filepath)).await?;
        let file = Arc::new(Mutex::new(file));

        // Write file data to the temporary file
        while let Some(chunk) = field.next().await {
            let data = chunk?;

            let file = Arc::clone(&file);

            let res = web::block(move || {
                let mut f = file.lock().unwrap();

                let file = f.as_mut().unwrap(); // Access the File instance from the MutexGuard

                file.write_all(&data)?;

                Ok::<_, std::io::Error>(())
            })
            .await?;

            if res.is_err() {
                return Ok(HttpResponse::InternalServerError().body("Error saving file"));
            }
        }

        // Load the JSON data from the file
        from_json(Some(&load_filepath));

        // Delete the file after processing
        if let Err(e) = std::fs::remove_file(&load_filepath) {
            eprintln!("Error deleting file: {}", e);
        };

        return Ok(HttpResponse::Ok().body("File uploaded and processed"));
    }

    // If no file was found in the request
    Ok(HttpResponse::BadRequest().body("No file uploaded"))
}

pub async fn download_json_file() -> impl Responder {
    with_database(move |_manager| {
        // get services
        let services_ = <DatabaseManager as Database<ServiceModel>>::get_many(_manager);

        // get screens
        let screens_ = <DatabaseManager as Database<ScreenModel>>::get_many(_manager);

        // Check if the file exists
        let filepath = "./uploads/data.json";

        async move {
            // Create a new USSDMenu instance
            let mut menu = USSDMenu::new();

            // Get services
            match services_ {
                Ok(data) => {
                    for service in data {
                        menu.services
                            .insert(service.name.clone(), service.to_ussd_service());
                    }
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Error getting services");
                }
            }

            // Get screens
            match screens_ {
                Ok(data) => {
                    for screen in data {
                        menu.menus
                            .insert(screen.name.clone(), screen.to_ussd_screen());
                    }
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Error getting screens");
                }
            }

            // Save the menu to a JSON file
            to_json(Some(filepath), menu);

            // Read the file
            let result = web::block(move || std::fs::read(filepath)).await.unwrap();

            match result {
                Ok(data) => {
                    // Convert data to Bytes
                    let body = actix_web::web::Bytes::from(data);

                    // Return the file as a response
                    HttpResponse::Ok()
                        .content_type("application/json")
                        .body(body)
                }
                Err(_) => HttpResponse::InternalServerError().body("Error reading file"),
            }
        }
    })
    .await
}
