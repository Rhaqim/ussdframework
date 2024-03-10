use actix_web::{web, App, HttpResponse, HttpServer};
use std::{collections::HashMap, sync::Mutex};
use ussdframework::prelude::*;

mod config;
mod functions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let session_store = InMemorySessionStore::new();
        let app = UssdApp::new("functions".to_string(), Box::new(session_store));

        let content = include_str!("../examples/data/menu.json");
        let menus: USSDMenu = serde_json::from_str(&content).unwrap();

        register_functions(functions::get_functions());

        App::new()
            .app_data(web::Data::new(app))
            .app_data(web::Data::new(menus))
            .route("/ussd", web::post().to(handle_ussd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn handle_ussd(
    req: web::Json<USSDRequest>,
    app: web::Data<UssdApp>,
    menus: web::Data<USSDMenu>,
) -> HttpResponse {
    let request = req.into_inner();
    let response = app.run(request, menus.get_ref().clone());
    HttpResponse::Ok().body(response.message)
}

pub struct InMemorySessionStore {
    data: Mutex<HashMap<String, String>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl SessionCache for InMemorySessionStore {
    fn store_session(&self, session: &USSDSession) -> Result<(), String> {
        let mut data = self.data.lock().unwrap();
        data.insert(
            session.session_id.clone(),
            serde_json::to_string(session).unwrap(),
        );
        Ok(())
    }

    fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
        let data = self.data.lock().unwrap();
        match data.get(session_id) {
            Some(session) => Ok(Some(serde_json::from_str(session).unwrap())),
            None => Ok(None),
        }
    }
}
