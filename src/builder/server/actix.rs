use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

use crate::error;

use crate::builder::DatabaseManager;

use crate::builder::api::screens;
use crate::builder::api::services;

const STATIC_DIR: &str = "src/builder/static";
const APP_DIR: &str = "src/builder/static/server/app";

pub async fn start_server(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        let db_manager = DatabaseManager::new();

        App::new()
            .app_data(web::Data::new(db_manager))
            // Serve the API
            .service(
                web::resource("/api/services")
                    .route(web::post().to(services::create))
                    .route(web::put().to(services::update))
                    .route(web::delete().to(services::delete))
                    .route(web::get().to(services::get_list))
                    // .route(web::get().to(services::get_multiple))
                    // .route(web::get().to(services::get_all)),
            )
            .service(
                web::resource("/api/screens")
                    .route(web::post().to(screens::create))
                    .route(web::put().to(screens::update))
                    .route(web::delete().to(screens::delete))
                    .route(web::get().to(screens::get_list))
                    // .route(web::get().to(screens::get_multiple))
                    // .route(web::get().to(screens::get_all)),
            )
            // Serve static files
            .service(Files::new("/_next", STATIC_DIR).index_file(format!("{}/index.html", APP_DIR)))
            // Route for other pages
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

async fn index(req: HttpRequest) -> Result<actix_files::NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();

    let full_path: String = match path.to_str() {
        Some(p) => match p {
            "" => format!("{}/index.html", APP_DIR),
            _ => format!("{}/{}.html", APP_DIR, p),
        },
        None => format!("{}/_not-found.html", APP_DIR),
    };

    let res = actix_files::NamedFile::open(full_path.clone());

    match res {
        Ok(file) => Ok(file),
        Err(e) => {
            error!("Error opening file: {:?} at path: {}", e, full_path);
            Ok(actix_files::NamedFile::open(format!(
                "{}/_not-found.html",
                APP_DIR
            ))?)
        }
    }
}
