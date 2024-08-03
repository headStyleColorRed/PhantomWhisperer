use crate::helpers::encoder;
use warp::reject::Rejection;
use crate::helpers::aprs_packet::AprsPacket;

pub async fn create_packet(data: AprsPacket) -> Result<impl warp::Reply, Rejection> {
    println!("[ENCODER] --> 1. Starting create_packet function");

    let encoded_packet: Vec<i16> = encoder::encode_message(&data.source, &data.destination, &data.information);
    println!("[ENCODER] --> 10. Message encoded, packet length: {}", encoded_packet.len());

    encoder::create_wav_file(encoded_packet)
}
