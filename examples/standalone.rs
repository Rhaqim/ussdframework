/// Standalone HTTP server example — no `menubuilder` feature required.
///
/// The developer provides the `USSDMenu` directly (loaded from JSON here, but it
/// can be built from any source).  A single `/ussd` POST endpoint processes
/// incoming USSD requests exactly as a real gateway would call it.
///
/// Run with:   make run-standalone
/// Then POST to http://localhost:8081/ussd
use std::sync::Arc;

use actix_web::{web, App, HttpResponse, HttpServer};
use ussdframework::prelude::*;

mod functions;

struct AppState {
    app: UssdApp,
    menu: USSDMenu,
}

async fn handle_ussd(
    state: web::Data<Arc<AppState>>,
    req: web::Json<USSDRequest>,
) -> HttpResponse {
    let response = state.app.run(req.into_inner(), state.menu.clone());
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the menu from JSON. In production this can come from any source.
    let menu = USSDMenu::load_from_json("examples/data/menu.json")
        .expect("Failed to load examples/data/menu.json");

    let mut app = UssdApp::new(true, None);
    app.register_functions(functions::get_functions());

    let state = Arc::new(AppState { app, menu });
    let state_data = web::Data::new(state);

    let port = 8081u16;
    println!("Standalone USSD server listening on http://127.0.0.1:{}", port);
    println!("POST http://127.0.0.1:{}/ussd", port);

    HttpServer::new(move || {
        App::new()
            .app_data(state_data.clone())
            .service(web::resource("/ussd").route(web::post().to(handle_ussd)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
