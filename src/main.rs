use warp::Filter;
mod helpers;
mod routes;


use routes::encoder::modulate_text;
use routes::decoder::decode_wav;

#[tokio::main]
async fn main() {
    // Define the route to serve the file with JSON data
    let encode_route = warp::path("encode")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(modulate_text);

    // Define the route for decoding WAV files
    let decode_route = warp::path("decode")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(decode_wav);

    // Define the route for the root path
    let root_route = warp::path::end().map(|| "Server is up and running");

    // Combine the routes
    let routes = warp::get().and(root_route)
        .or(encode_route)
        .or(decode_route);

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
