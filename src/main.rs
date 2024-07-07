use bytes::Buf;
use warp::Filter;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;
use helpers::errors::CustomError;
mod helpers;

#[derive(Deserialize, Serialize)]
struct RequestData {
    message: String,
}

#[derive(Serialize)]
struct DecodedResponse {
    decoded_message: String,
}

#[tokio::main]
async fn main() {
    // Route that encodes a message and modulates it into a WAV file
    let encode_route = warp::path("encode")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(modulate_text);

    // Route that decodes a WAV file and returns the decoded message
    let decode_route = warp::path("decode")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(decode_wav);

    // Route for the root path
    let root_route = warp::path::end().map(|| "Server is up and running");

    // Combine the routes
    let routes = warp::get().and(root_route)
        .or(encode_route)
        .or(decode_route);

    // CORS middleware, TODO: Remove this in production
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["POST", "GET"]);

    // Apply CORS to our routes
    let routes = routes.with(cors);

    // Notify the user that the server is running
    println!("Starting server on http://localhost:3030");

    // Start the warp server on port 3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn modulate_text(data: RequestData) -> Result<impl warp::Reply, warp::Rejection> {
    // Encode the message
    let encoded_path = helpers::encoder::encode_message(&data.message, "src/files/encoded_message.txt")
        .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

    // Modulate the file
    let modulated_file = "src/files/modulated_message.wav";
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

async fn decode_wav(mut form: warp::multipart::FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let mut wav_data = Vec::new();

    while let Some(part) = form.try_next().await.map_err(|e| warp::reject::custom(CustomError(e.to_string())))? {
        if part.name() == "file" {
            wav_data = part.stream()
                .try_fold(Vec::new(), |mut acc, chunk| async move {
                    acc.extend_from_slice(chunk.chunk());
                    Ok(acc)
                })
                .await
                .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;
            break;
        }
    }

    if wav_data.is_empty() {
        return Err(warp::reject::custom(CustomError("No WAV file found in the request".to_string())));
    }

    // Save the WAV data to a temporary file
    let temp_wav_file = "temp_decoded.wav";
    tokio::fs::write(temp_wav_file, &wav_data).await
        .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

    // Decode the file
    let decoded_message: String = helpers::decoder::decode_file(temp_wav_file)
        .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;

    // Clean up temporary files
    tokio::fs::remove_file(temp_wav_file).await.ok();

    // Return the decoded message as JSON
    let response = DecodedResponse { decoded_message };
    Ok(warp::reply::json(&response))
}
