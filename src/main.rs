use std::{error::Error, fmt};

use warp::Filter;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use serde::{Deserialize, Serialize};
mod helpers;

#[derive(Deserialize, Serialize)]
struct RequestData {
    message: String,
}

#[tokio::main]
async fn main() {
    // Define the route to serve the file with JSON data
    let file_route = warp::path("encode")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(modulate_text);

    // Define the route for the root path
    let root_route = warp::path::end().map(|| "Server is up and running");

    // Combine the routes
    let routes = warp::get().and(root_route).or(file_route);

        // Add CORS support
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["POST", "GET"]);

    // Apply CORS to our routes
    let routes = routes.with(cors);

    // Start the warp server on port 3030
    println!("Starting server on http://localhost:3030");

    // Start the warp server on port 3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn modulate_text(data: RequestData) -> Result<impl warp::Reply, warp::Rejection> {
    // Encode the message
    let encoded_path = helpers::encoder::encode_message(&data.message,  "encoded_message.txt")
        .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

    // Modulate the file
    let modulated_file = "modulated_message.wav";
    helpers::modulator::modulate_file(&encoded_path, modulated_file)
        .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

    // Read the modulated file contents
    let mut file = File::open(modulated_file).await.map_err(|_| warp::reject::not_found())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.map_err(|_| warp::reject::not_found())?;

    // Return the file contents as a response
    Ok(warp::http::Response::builder()
        .header("Content-Type", "audio/wav")
        .body(buffer))
}

// Custom error type for error handling
#[derive(Debug)]
struct CustomError(String);

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom error: {}", self.0)
    }
}

impl warp::reject::Reject for CustomError {}
