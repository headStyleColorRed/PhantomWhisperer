use warp::Filter;
use std::path::Path;
mod helpers;
mod routes;

use routes::encoder::modulate_text;
use routes::decoder::decode_wav;

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

    // Route that will encode a message and return a WAV file
    let encode_route = warp::path("encode")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(modulate_text);

    // Route that will decode a WAV file and return the message
    let decode_route = warp::path("decode")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(decode_wav);

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
    println!("Starting server on http://localhost:3030");

    // Start the warp server on port 3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
