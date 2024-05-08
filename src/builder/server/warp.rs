use warp::{filters::fs::File, Filter, Rejection, Reply};

use serde::{Deserialize, Serialize};

use crate::info;

// Define a struct to represent data
#[derive(Debug, Serialize, Deserialize)]
struct Data {
    id: u32,
    name: String,
    // Add other fields as needed
}

// Endpoint to fetch data
pub fn fetch_data() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "data").and(warp::get()).map(|| {
        // Simulated data fetching from the database
        let data = vec![
            Data {
                id: 1,
                name: "Item 1".to_string(),
            },
            Data {
                id: 2,
                name: "Item 2".to_string(),
            },
        ];
        warp::reply::json(&data)
    })
}

// Endpoint to add data
pub fn add_data() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "data")
        .and(warp::post())
        .and(warp::body::json())
        .map(|new_data: Data| {
            // Simulated data addition to the database
            println!("Added data: {:?}", new_data);
            warp::reply::json(&new_data)
        })
}

// Define your API routes
// ...

// Serve static files
// fn static_files() -> impl Filter<Extract = (File,), Error = Rejection> + Clone {
//     warp::path("static").and(warp::fs::dir("./static"))
// }
fn static_files() -> impl Filter<Extract = (File,), Error = Rejection> + Clone {
    warp::fs::dir("./static")
}

fn home_page() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("admin").and(warp::get()).and(warp::fs::file("./static/server/app/admin.html"))
}

pub async fn start_server(port: u16) {
    info!("Starting server on port: {}", port);
    // Run the build script before starting the server
    // cargo_make::run_task("build").expect("Failed to run build script");

    // Define routes
    let routes = fetch_data().or(add_data()).or(static_files()).or(home_page());
    info!("Server started successfully");

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
