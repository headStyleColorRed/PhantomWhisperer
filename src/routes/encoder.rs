use warp::Filter;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use serde::{Deserialize, Serialize};
use crate::helpers::errors::CustomError;
use crate::helpers;

#[derive(Deserialize, Serialize)]
pub struct RequestData {
    pub message: String,
}

pub async fn modulate_text(data: RequestData) -> Result<impl warp::Reply, warp::Rejection> {
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
