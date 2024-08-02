use crate::helpers::decoder;
use crate::helpers::errors::CustomError;
use bytes::Buf;
use futures::TryStreamExt;
use hound::WavReader;
use warp::reject::Rejection;

pub async fn decode_audio(
    mut form: warp::multipart::FormData,
) -> Result<impl warp::Reply, Rejection> {
    println!("[DECODER] --> 1. Starting decode_audio function");

    // Extract the uploaded file from the form data
    let part = form
        .try_next()
        .await
        .map_err(|e| warp::reject::custom(CustomError(format!("Form data error: {}", e))))?
        .ok_or_else(|| warp::reject::custom(CustomError("No file uploaded".to_string())))?;

    println!("[DECODER] --> 2. File part extracted from form data");

    let file_bytes: Vec<u8> = part
        .stream()
        .try_fold(Vec::new(), |mut acc, chunk| async move {
            acc.extend_from_slice(chunk.chunk());
            Ok(acc)
        })
        .await
        .map_err(|e| warp::reject::custom(CustomError(format!("File read error: {}", e))))?;

    println!("[DECODER] --> 3. File bytes read, size: {} bytes", file_bytes.len());

    // Create a cursor from the file bytes
    let cursor = std::io::Cursor::new(file_bytes);

    // Create a WavReader from the cursor
    let mut reader = WavReader::new(cursor)
        .map_err(|e| warp::reject::custom(CustomError(format!("WAV parsing error: {}", e))))?;

    println!("[DECODER] --> 4. WAV file parsed successfully");

    // Read the samples into a Vec<i16>
    let samples: Vec<i16> = reader
        .samples::<i16>()
        .collect::<Result<Vec<i16>, _>>()
        .map_err(|e| warp::reject::custom(CustomError(format!("Sample reading error: {}", e))))?;

    println!("[DECODER] --> 5. Samples read, count: {}", samples.len());

    let decoded_message = decoder::decode_audio(samples.as_slice())
        .map_err(|e| warp::reject::custom(CustomError(format!("Decoding error: {}", e))))?;

    println!("[DECODER] --> 6. Audio decoded successfully");

    Ok(warp::reply::json(&decoded_message))
}
