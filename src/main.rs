// src/main.rs

use actix_web::{web, App, HttpRequest, HttpServer, Result};
use actix_files::Files;
use std::path::PathBuf;

// async fn index(_req: HttpRequest) -> Result<actix_files::NamedFile> {
//     Ok(actix_files::NamedFile::open("./static/server/app/index.html")?) // Adjust path as needed
// }

async fn index(_req: HttpRequest) -> Result<actix_files::NamedFile> {
    let path: PathBuf = "./static/index.html".into(); // Adjust path as needed
    println!("Attempting to open file: {:?}", path); // Log the resolved file path
    // Ok(actix_files::NamedFile::open(path)?)
    let result = actix_files::NamedFile::open("static/index.html");

    match result {
        Ok(file) => Ok(file),
        Err(e) => {
            println!("Error opening file: {:?}", e);
            // print current directory
            let current_dir = std::env::current_dir().unwrap();
            println!("The current directory is {}", current_dir.display());
            Ok(actix_files::NamedFile::open("static/404.html")?)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/data").route(web::get().to(get_data)))
            .default_service(web::get().to(index)) // Serve the index.html for all other routes
            .service(Files::new("/", "./static").index_file("index.html")) // Serve all files under the static directory
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
