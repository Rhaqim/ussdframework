use std::path::Path;
use std::path::PathBuf;
use actix_files::Files;

const STATIC_DIR: &str = "src/builder/static";
const APP_DIR: &str = "src/builder/static/server/app";

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


macro_rules! define_entity_routes {
    ($entity:ident, $entity_path:expr, $entity_module:ident) => {
        .service(
            web::resource($entity_path)
                .route(web::post().to($entity_module::create))
                .route(web::put().to($entity_module::update))
                .route(web::get().to($entity_module::get_all)),
        )
        .service(
            web::resource(format!("{}/{{id}}", $entity_path))
                .route(web::get().to($entity_module::get))
                .route(web::delete().to($entity_module::delete)),
        )
        .service(
            web::resource(format!("{}/multiple", $entity_path))
                .route(web::post().to($entity_module::get_multiple)),
        )
    };
}

fn main() {
    // Define routes for services
    let services_routes = define_entity_routes!(services, "/api/services", services);

    // Define routes for screens
    let screens_routes = define_entity_routes!(screens, "/api/screens", screens);

    // Define routes for menu items
    let menu_items_routes = define_entity_routes!(menu_items, "/api/menu_items", menu_items);

    // Define routes for router options
    let router_options_routes =
        define_entity_routes!(router_options, "/api/router_options", router_options);

    // Combine routes
    let routes = services_routes
        .service(screens_routes)
        .service(menu_items_routes)
        .service(router_options_routes);

    // Add routes to Actix Web App
    HttpServer::new(|| {
        App::new().service(routes)
    })
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to address")
    .run()
    .expect("Failed to start server");
}
