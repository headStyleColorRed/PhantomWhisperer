use warp::Filter;
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;
use bytes::Buf;
use crate::helpers::errors::CustomError;
use crate::helpers;

#[derive(Serialize)]
pub struct DecodedResponse {
    pub decoded_message: String,
}

pub async fn decode_wav(mut form: warp::multipart::FormData) -> Result<impl warp::Reply, warp::Rejection> {
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
