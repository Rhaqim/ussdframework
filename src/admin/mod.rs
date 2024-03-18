pub mod api;

use api::get_data;

use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/data").route(web::get().to(get_data)))
            .default_service(web::get().to(index)) // Serve the index.html for all other routes
            .service(Files::new("/", "./_next").index_file("/server/app/index.html"))
        // Serve all files under the static directory
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index(_req: HttpRequest) -> Result<actix_files::NamedFile> {
    let path: PathBuf = "./_next/server/app/index.html".into(); // Adjust path as needed
    println!("Attempting to open file: {:?}", path); // Log the resolved file path
    Ok(actix_files::NamedFile::open(path)?)
}
