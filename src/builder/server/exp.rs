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
