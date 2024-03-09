use actix_web::{web, App, HttpResponse, HttpServer};
use std::{collections::HashMap, sync::Mutex};
use ussdframework::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let session_store = InMemorySessionStore::new();
        let app = UssdApp::new("config/functions".to_string(), Box::new(session_store));

        let content = include_str!("../examples/data/menu.json");
        let menus: USSDMenu = serde_json::from_str(&content).unwrap();

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
    HttpResponse::Ok().json(response)
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

//     // We create a TcpListener and bind it to 127.0.0.1:3000
//     let listener = TcpListener::bind(addr).await?;

//     // We start a loop to continuously accept incoming connections
//     loop {
//         let (stream, _) = listener.accept().await?;

//         // Use an adapter to access something implementing `tokio::io` traits as if they implement
//         // `hyper::rt` IO traits.
//         let io = TokioIo::new(stream);

//         // Spawn a tokio task to serve multiple connections concurrently
//         tokio::task::spawn(async move {
//             // Finally, we bind the incoming connection to our `hello` service
//             if let Err(err) = http1::Builder::new()
//                 // `service_fn` converts our function in a `Service`
//                 .serve_connection(io, service_fn(hello))
//                 .await
//             {
//                 println!("Error serving connection: {:?}", err);
//             }
//         });
//     }
// }

// async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
//     Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
// }

// fn setup() {
//     let session_store = InMemorySessionStore::new();
//     let app = UssdApp::new("config/functions".to_string(), Box::new(session_store));

//     let content = include_str!("../examples/data/menu.json");
//     let menus: USSDMenu = serde_json::from_str(&content).unwrap();

//     loop {
//         // Define input variable for each iteration
//         let mut input = String::new();

//         let request = USSDRequest {
//             session_id: "1234".to_string(),
//             msisdn: "1234".to_string(),
//             input: input.clone(), // Use input directly here
//             service_code: "1234".to_string(),
//             language: "en".to_string(),
//         };

//         let response = app.run(request, menus.clone());
//         app.display_menu(&response);

//         // Process user input and update request input for the next iteration
//         // For demonstration purposes, let's assume the input is obtained from user input
//         println!("Enter your choice:");
//         std::io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         let trimmed_input = input.trim().to_string();

//         // Break out of the loop if the user chooses to exit
//         if trimmed_input == "exit" {
//             break;
//         }
//     }
// }

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
