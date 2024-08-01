use crate::helpers::encoder;
use serde::{Deserialize, Serialize};
use warp::reject::Rejection;
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[derive(Deserialize, Serialize)]
pub struct RequestData {
    pub destination: String,
    pub source: String,
    pub information: String
}

// This method will be called when the client sends a POST request to /encoder
// The request body should contain a JSON object with the following fields:
// - destination: the destination callsign
// - source: the source callsign
// - information: the message to be transmitted
// The method will return a JSON object with the encoded packet and its length
pub async fn create_packet(data: RequestData) -> Result<impl warp::Reply, Rejection> {
    // Print the request data for debug reasons
    print_request_data(&data);

    // Encode the message
    let encoded_packet: Vec<Vec<u8>> = encoder::encode_message(&data.information, &data.source, &data.destination);

    // Print the encoded packet for debug reasons
    print_encoded_packet(encoded_packet.clone());


    // Flatten the Vec<Vec<u8>> into a single Vec<u8> so we can send it as a response
    let flattened_packet: Vec<u8> = encoded_packet.into_iter().flatten().collect();

    // Here, instead of creating a WAV file, we would interface with the radio
    // For now, we'll just return the encoded packet as a response
    Ok(warp::reply::json(&serde_json::json!({
        "encoded_packet": STANDARD.encode(flattened_packet.clone()),
        "length": flattened_packet.len(),
    })))
}

// Request data debug function
pub fn print_request_data(data: &RequestData) {
    println!("");
    println!("REQUEST DATA:");
    println!("_______________");
    println!("Destination: {}", data.destination);
    println!("Source: {}", data.source);
    println!("Information: {}", data.information);
}

// Debug function with nice printing for terminal
pub fn print_encoded_packet(encoded_packet: Vec<Vec<u8>>) {
    println!("");
    println!("RESPONSE DATA:");
    println!("_______________");
    for packet in encoded_packet {
        println!("Packet: {:?}", packet);
    }
}
