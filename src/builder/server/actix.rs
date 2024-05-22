use actix_web::HttpResponse;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use awc::Client;

use crate::builder::api::file;
use crate::builder::api::menu_items;
use crate::builder::api::router_options;
use crate::builder::api::screens;
use crate::builder::api::services;

use crate::error;

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
            // Download
            .service(web::resource("/api/download").route(web::get().to(file::download_json_file)))
            // Serve static files
            // .service(Files::new("/_next", STATIC_DIR).index_file(format!("{}/index.html", APP_DIR)))
            // Route for other pages
            // .route("/{filename:.*}", web::get().to(index))
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
