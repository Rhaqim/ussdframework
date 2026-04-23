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
    // Build shared state ONCE, outside the closure.
    // HttpServer calls the closure once per worker thread, so any state created
    // inside the closure would not be shared across threads — session lookups
    // from a second thread would always fail, causing a fresh session (and a
    // repeated MainMenu) on every other request.
    let session_store = InMemorySessionStore::new();
    let mut app = UssdApp::new(false, Some(Box::new(session_store)));
    app.register_functions(functions::get_functions());

    let content = include_str!("../examples/data/menu.json");
    let menus: USSDMenu = serde_json::from_str(content).unwrap();

    // web::Data wraps in Arc — cloning it inside the closure shares the same
    // underlying UssdApp (and its session store) across all worker threads.
    let app_data = web::Data::new(app);
    let menus_data = web::Data::new(menus);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(menus_data.clone())
            .service(health_check)
            .route("/ussd", web::post().to(handle_ussd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
