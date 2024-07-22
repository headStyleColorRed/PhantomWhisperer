use serde::Serialize;
use futures::TryStreamExt;
use bytes::Buf;
use crate::helpers::errors::CustomError;
use crate::helpers;
use warp::reject::Rejection;

#[derive(Serialize)]
pub struct DecodedResponse {
    pub decoded_message: String,
}

pub async fn decode_wav(mut form: warp::multipart::FormData) -> Result<impl warp::Reply, Rejection> {
    let mut wav_data = Vec::new();
    println!("[DECODER ROUTE] Starting to process form data...");

    while let Some(part) = form.try_next().await.map_err(|e| warp::reject::custom(CustomError(e.to_string())))? {
        println!("[DECODER ROUTE] Processing part with name: {}", part.name());

        if part.name() == "file" {
            println!("[DECODER ROUTE] Found file part.");
            wav_data = part.stream()
                .try_fold(Vec::new(), |mut acc, chunk| async move {
                    acc.extend_from_slice(chunk.chunk());
                    Ok(acc)
                })
                .await
                .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;
            println!("[DECODER ROUTE] WAV data collected, length: {}", wav_data.len());
            break;
        }
    }

    if wav_data.is_empty() {
        println!("[DECODER ROUTE] No WAV file found in the request.");
        return Err(warp::reject::custom(CustomError("No WAV file found in the request".to_string())));
    }

    // Save the WAV data to a temporary file
    let temp_wav_file = "temp_decoded.wav";
    println!("[DECODER ROUTE] Saving WAV data to temporary file: {}", temp_wav_file);
    tokio::fs::write(temp_wav_file, &wav_data).await
        .map_err(|e| warp::reject::custom(CustomError(e.to_string())))?;
    println!("[DECODER ROUTE] Temporary file saved.");

    // Decode the file
    println!("[DECODER ROUTE] Decoding the file...");
    let result = helpers::decoder::decode_file(temp_wav_file)
        .map_err(|e| {
            println!("[DECODER ROUTE] Decoding failed: {:?}", e);
            warp::reject::custom(CustomError(e.to_string()))
        })
        .and_then(|decoded_message| {
            println!("[DECODER ROUTE] Decoding complete.");
            Ok(warp::reply::json(&DecodedResponse { decoded_message }))
        });

    // Clean up temporary files
    if let Err(e) = tokio::fs::remove_file(temp_wav_file).await {
        println!("[DECODER ROUTE] Failed to remove temporary file: {:?}", e);
    } else {
        println!("[DECODER ROUTE] Temporary file removed.");
    }

    result
}
