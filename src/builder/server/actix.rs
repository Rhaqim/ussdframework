use std::sync::Arc;

use actix_web::HttpResponse;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use awc::Client;

use crate::builder::api::file;
use crate::builder::api::menu_items;
use crate::builder::api::router_options;
use crate::builder::api::screens;
use crate::builder::api::services;
use crate::builder::file::build;
use crate::core::{process_request, InMemorySessionStore, SessionCache, USSDRequest};
use crate::types::FunctionMap;

use crate::error;

async fn handle_ussd(
    req: web::Json<USSDRequest>,
    session_cache: web::Data<Arc<Box<dyn SessionCache>>>,
    function_map: web::Data<FunctionMap>,
    json_seed: web::Data<Option<String>>,
) -> HttpResponse {
    let menus = build(json_seed.as_deref());
    let response = process_request(&req.into_inner(), session_cache.as_ref(), &menus, &function_map);
    HttpResponse::Ok().json(response)
}

pub async fn start_server(
    port: u16,
    function_map: FunctionMap,
    json_seed: Option<String>,
    database_url: Option<String>,
) -> std::io::Result<()> {
    // Set the database URL env var before any DatabaseManager is created.
    // All internal calls to establish_connection() / establish_pool() pick this up.
    if let Some(ref url) = database_url {
        std::env::set_var("USSD_DATABASE_URL", url);
    }

    let session_store: Arc<Box<dyn SessionCache>> =
        Arc::new(Box::new(InMemorySessionStore::new()));
    let session_data = web::Data::new(session_store);
    let function_data = web::Data::new(function_map);
    let seed_data = web::Data::new(json_seed);

    HttpServer::new(move || {
        App::new()
            .app_data(session_data.clone())
            .app_data(function_data.clone())
            .app_data(seed_data.clone())
            // Services
            .service(
                web::resource("/api/services")
                    .route(web::post().to(services::create))
                    .route(web::put().to(services::update))
                    .route(web::get().to(services::get_all)),
            )
            .service(
                web::resource("/api/services/multiple")
                    .route(web::get().to(services::get_multiple)),
            )
            .service(
                web::resource("/api/services/{name}")
                    .route(web::get().to(services::get))
                    .route(web::delete().to(services::delete)),
            )
            // Screens
            .service(
                web::resource("/api/screens")
                    .route(web::post().to(screens::create))
                    .route(web::put().to(screens::update))
                    .route(web::get().to(screens::get_all)),
            )
            .service(
                web::resource("/api/screens/multiple").route(web::get().to(screens::get_multiple)),
            )
            .service(
                web::resource("/api/screens/{name}")
                    .route(web::get().to(screens::get))
                    .route(web::delete().to(screens::delete)),
            )
            // MenuItems
            .service(
                web::resource("/api/menu_items")
                    .route(web::post().to(menu_items::create))
                    .route(web::put().to(menu_items::update))
                    .route(web::get().to(menu_items::get_all)),
            )
            .service(
                web::resource("/api/menu_items/multiple")
                    .route(web::get().to(menu_items::get_multiple)),
            )
            .service(
                web::resource("/api/menu_items/{name}")
                    .route(web::get().to(menu_items::get))
                    .route(web::delete().to(menu_items::delete)),
            )
            // Router Options
            .service(
                web::resource("/api/router_options")
                    .route(web::post().to(router_options::create))
                    .route(web::put().to(router_options::update))
                    .route(web::get().to(router_options::get_all)),
            )
            .service(
                web::resource("/api/router_options/multiple")
                    .route(web::get().to(router_options::get_multiple)),
            )
            .service(
                web::resource("/api/router_options/{name}")
                    .route(web::get().to(router_options::get))
                    .route(web::delete().to(router_options::delete)),
            )
            // File Upload
            .service(web::resource("/api/upload").route(web::post().to(file::process_json_file)))
            // Download
            .service(web::resource("/api/download").route(web::get().to(file::download_json_file)))
            // USSD request handler
            .service(web::resource("/ussd").route(web::post().to(handle_ussd)))
            // Proxy all other requests to Next.js
            .default_service(web::route().to(proxy_to_next_server))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

async fn proxy_to_next_server(req: HttpRequest) -> Result<HttpResponse> {
    let client = Client::default();

    let mut new_req = client
        .request_from(format!("http://localhost:3000{}", req.uri()), req.head())
        .no_decompress();

    for (key, value) in req.headers() {
        new_req = new_req.append_header((key.clone(), value.clone()));
    }

    let response = new_req.send().await;

    match response {
        Ok(mut response) => {
            let mut client_resp = &mut HttpResponse::build(response.status());

            for (key, value) in response.headers() {
                client_resp = client_resp.append_header((key.clone(), value.clone()));
            }

            Ok(client_resp.body(response.body().await?))
        }
        Err(e) => {
            error!("Error proxying request: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
