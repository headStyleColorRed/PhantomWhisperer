use warp::Filter;
use std::path::Path;
mod helpers;
mod routes;

#[cfg(test)]
mod tests;

use routes::encoder::create_packet;
use routes::decoder::decode_audio;
use helpers::errors::handle_rejection;

#[tokio::main]
async fn main() {
    // Get the canonical path to the web directory
    let web_dir = Path::new("web").canonicalize().expect("web directory not found");

    // Route to serve the index.html file at the root
    let index_route = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(web_dir.join("index.html")));

    // Route to serve static files from the web directory
    let static_route = warp::path("static")
        .and(warp::fs::dir(web_dir));

    // Route that will encode a message and return the packet encoded
    let encode_route = warp::path("encode")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_packet);

    // Route that will decode a WAV file and return the message
    let decode_route = warp::path("decode")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(decode_audio)
        .recover(handle_rejection);

    // Route to confirm the server is up and running
    let health_route = warp::path("health").map(|| "Server is up and running");

    // Combine the routes
    let routes = index_route
        .or(static_route)
        .or(encode_route)
        .or(decode_route)
        .or(health_route);

    // Add CORS support, TODO: Add proper configuration for production
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["POST"]);

    // Apply CORS to our routes
    let routes = routes.with(cors);

    // Notify the user that the server is running
    println!("Starting server on http://0.0.0.0:3030");

    // Start the warp server on port 3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030)) // Should be 0.0.0.0 for docker
        .await;
}
