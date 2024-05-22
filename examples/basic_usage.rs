use actix_web::{web, App, HttpServer};
use ussdframework::prelude::*;

mod config;
mod controller;
mod functions;
mod session;

use controller::{handle_ussd, health_check};
use session::InMemorySessionStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the MenuBuilder server
    menubuilder::MenuBuilder::server(8080).await?;
    // menubuilder::MenuBuilder::to_json(Some("menu.json"));

    HttpServer::new(move || {
        let session_store = InMemorySessionStore::new();

        // Create a new instance of UssdApp
        let app = UssdApp::new(false, Some(Box::new(session_store)));

        // Register functions
        app.register_functions(functions::get_functions());

        // Load menus
        let content = include_str!("../examples/data/menu.json");
        let menus: USSDMenu = serde_json::from_str(&content).unwrap();

        // Create a new instance of the Actix web application
        App::new()
            .app_data(web::Data::new(app))
            .app_data(web::Data::new(menus))
            .service(health_check)
            .route("/ussd", web::post().to(handle_ussd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
