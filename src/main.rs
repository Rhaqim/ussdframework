// src/main.rs

use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

// async fn index(_req: HttpRequest) -> Result<actix_files::NamedFile> {
//     Ok(actix_files::NamedFile::open("./static/server/app/index.html")?) // Adjust path as needed
// }

async fn index(_req: HttpRequest) -> Result<actix_files::NamedFile> {
    let path: PathBuf = "./_next/server/app/index.html".into(); // Adjust path as needed
                                                                // Ok(actix_files::NamedFile::open(path)?)
    let result = actix_files::NamedFile::open(path);

    match result {
        Ok(file) => Ok(file),
        Err(e) => {
            println!("Error opening file: {:?}", e);
            Ok(actix_files::NamedFile::open("_next/404.html")?)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/data").route(web::get().to(get_data)))
            .default_service(web::get().to(index)) // Serve the index.html for all other routes
            .service(Files::new("/_next", "./_next").index_file("server/app/index.html"))
        // Serve all files under the static directory
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

///  The  get_data  function is a simple function that returns a string.
///  The  index  function is a simple function that returns a file.
///  The  main  function is the entry point of the application. It creates a new  HttpServer  and binds it to the address
async fn get_data() -> Result<String> {
    Ok("Some data".to_string())
}
