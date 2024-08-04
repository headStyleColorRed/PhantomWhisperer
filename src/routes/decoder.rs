use crate::helpers::decoder;
use crate::helpers::errors::CustomError;
use warp::reject::Rejection;

pub async fn decode_audio(form: warp::multipart::FormData) -> Result<impl warp::Reply, Rejection> {
    println!("[DECODER] --> 1. Starting decode_audio function");
    let samples = decoder::extract_wav_from_multipart(form)
        .await
        .map_err(|_| warp::reject::custom(CustomError(format!("WAV extraction error"))))?;

    let decoded_message = decoder::decode_audio(samples.as_slice())
        .map_err(|e| warp::reject::custom(CustomError(format!("Decoding error: {}", e))))?;

    println!("[DECODER] --> 14. Audio decoded successfully");

    println!("---------------------------------------");
    println!(" Source: {}", decoded_message.source);
    println!(" Destination: {}", decoded_message.destination);
    println!(" Digipeaters: {:?}", decoded_message.digipeaters);
    println!(" Information: {}", decoded_message.information);
    println!("---------------------------------------");

    Ok(warp::reply::json(&decoded_message))
}
