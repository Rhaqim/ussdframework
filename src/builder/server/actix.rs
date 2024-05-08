use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use static_dir::static_dir;
use std::path::PathBuf;

use crate::error;

use crate::builder::api::database::database_start;

const STATIC_DIR: &str = "src/builder/static";

const APP_DIR: &str = "src/builder/static/server/app/";

pub async fn start_server(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/data").route(web::get().to(database_start)))
            // Serve static files
            .service(Files::new("/_next", STATIC_DIR).index_file(format!("{}index.html", APP_DIR)))
            // Route for other pages
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

async fn index(req: HttpRequest) -> Result<actix_files::NamedFile> {
    // let raw_path = static_dir!("src/builder/static/server/app");

    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let res = actix_files::NamedFile::open(format!("{}/{}.html", APP_DIR, path.display()));
    match res {
        Ok(file) => Ok(file),
        Err(e) => {
            println!("Error opening file: {:?}", e);
            Ok(actix_files::NamedFile::open(format!("{}/_not-found.html", APP_DIR))?)
        }
    }
}

async fn _index(_req: HttpRequest) -> Result<actix_files::NamedFile> {
    // let path: PathBuf = "./_next/server/app/index.html".into(); // Adjust path as needed
    let path: PathBuf = "./frontend/.next/server/app/index.html".into(); // Adjust path as needed
    println!("Attempting to open file: {:?}", path); // Log the resolved file path
    let result = actix_files::NamedFile::open(path.clone());

    match result {
        Ok(file) => Ok(file),
        Err(e) => {
            error!("Error opening file: {:?} at path {}", e, path.display());
            Ok(actix_files::NamedFile::open("_next/404.html")?)
        }
    }
}
