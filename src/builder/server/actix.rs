use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use std::path::Path;
use std::path::PathBuf;

use crate::builder::api::file;
use crate::builder::api::menu_items;
use crate::builder::api::router_options;
use crate::builder::api::screens;
use crate::builder::api::services;

use crate::{debug, error};

const STATIC_DIR: &str = "src/builder/static";
const APP_DIR: &str = "src/builder/static/server/app";

pub async fn start_server(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Serve the API
            // Services
            .service(
                web::resource("/api/services")
                    .route(web::post().to(services::create))
                    .route(web::put().to(services::update))
                    .route(web::get().to(services::get_all)),
            )
            .service(
                web::resource("/api/services/{name}")
                    .route(web::get().to(services::get))
                    .route(web::delete().to(services::delete)),
            )
            .service(
                web::resource("/api/services/multiple")
                    .route(web::post().to(services::get_multiple)),
            )
            // Screens
            .service(
                web::resource("/api/screens")
                    .route(web::post().to(screens::create))
                    .route(web::put().to(screens::update))
                    .route(web::get().to(screens::get_all)),
            )
            .service(
                web::resource("/api/screens/{name}")
                    .route(web::get().to(screens::get))
                    .route(web::delete().to(screens::delete)),
            )
            .service(
                web::resource("/api/screens/multiple").route(web::post().to(screens::get_multiple)),
            )
            // MenuItems
            .service(
                web::resource("/api/menu_items")
                    .route(web::post().to(menu_items::create))
                    .route(web::put().to(menu_items::update))
                    .route(web::get().to(menu_items::get_all)),
            )
            .service(
                web::resource("/api/menu_items/{name}")
                    .route(web::get().to(menu_items::get))
                    .route(web::delete().to(menu_items::delete)),
            )
            .service(
                web::resource("/api/menu_items/multiple")
                    .route(web::post().to(menu_items::get_multiple)),
            )
            // Router Options
            .service(
                web::resource("/api/router_options")
                    .route(web::post().to(router_options::create))
                    .route(web::put().to(router_options::update))
                    .route(web::get().to(router_options::get_all)),
            )
            .service(
                web::resource("/api/router_options/{name}")
                    .route(web::get().to(router_options::get))
                    .route(web::delete().to(router_options::delete)),
            )
            .service(
                web::resource("/api/router_options/multiple")
                    .route(web::post().to(router_options::get_multiple)),
            )
            // File Upload
            .service(web::resource("/api/upload").route(web::post().to(file::process_json_file)))
            // Serve static files
            .service(Files::new("/_next", STATIC_DIR).index_file(format!("{}/index.html", APP_DIR)))
            // Route for other pages
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

async fn index(req: HttpRequest) -> Result<actix_web::HttpResponse> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let full_path = resolve_path(path.to_str().unwrap_or(""));

    let res = actix_files::NamedFile::open(full_path.clone());

    match res {
        Ok(file) => Ok(file.into_response(&req)),
        Err(e) => {
            error!("Error opening file: {:?} at path: {}", e, full_path);
            // Fallback to _not-found.html
            Ok(
                actix_files::NamedFile::open(format!("{}/_not-found.html", APP_DIR))?
                    .into_response(&req),
            )
        }
    }
}

fn resolve_path(requested_path: &str) -> String {
    if requested_path.is_empty() {
        format!("{}/index.html", APP_DIR)
    } else {
        let path = format!("{}/{}.html", APP_DIR, requested_path);
        if Path::new(&path).exists() {
            path
        } else if is_dynamic_route(requested_path) {
            let returned_path = format!("{}/{}.js", APP_DIR, "page");
            debug!("Dynamic route path: {:?}", returned_path);
            returned_path
        } else {
            path
        }
    }
}

fn is_dynamic_route(path: &str) -> bool {
    debug!("Its a dynamic route {:?}", path);

    // Check if the path matches a dynamic route structure
    // let dynamic_segments = vec!["[id]", "[slug]", "[name]"]; // Add more dynamic segments as needed
    let dynamic_segments = vec![
        "/services/",
        "/screens/",
        "/menu_items/",
        "/router_options/",
    ]; // Add more dynamic segments as needed
    dynamic_segments
        .iter()
        .any(|segment| path.contains(segment))
}

// async fn index(req: HttpRequest) -> Result<actix_files::NamedFile> {
//     let path: PathBuf = req.match_info().query("filename").parse().unwrap();

//     let full_path: String = match path.to_str() {
//         Some(p) => match p {
//             "" => format!("{}/index.html", APP_DIR),
//             _ => format!("{}/{}.html", APP_DIR, p),
//         },
//         None => format!("{}/_not-found.html", APP_DIR),
//     };

//     let res = actix_files::NamedFile::open(full_path.clone());

//     match res {
//         Ok(file) => Ok(file),
//         Err(e) => {
//             error!("Error opening file: {:?} at path: {}", e, full_path);
//             Ok(actix_files::NamedFile::open(format!(
//                 "{}/_not-found.html",
//                 APP_DIR
//             ))?)
//         }
//     }
// }
