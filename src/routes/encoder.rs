use crate::helpers::encoder;
use std::io::Cursor;
use warp::reject::Rejection;
use crate::helpers::aprs_packet::AprsPacket;
use hound::{WavWriter, WavSpec};
use crate::helpers::constants::*;

pub async fn create_packet(data: AprsPacket) -> Result<impl warp::Reply, Rejection> {
    println!("[ENCODER] --> 1. Starting create_packet function");

    let encoded_packet: Vec<i16> = encoder::encode_message(&data.source, &data.destination, &data.information);
    println!("[ENCODER] --> 10. Message encoded, packet length: {}", encoded_packet.len());

    create_wav_file(encoded_packet)
}

fn create_wav_file(audio_data: Vec<i16>) -> Result<impl warp::Reply, Rejection> {
    println!("[ENCODER] --> 11. Starting create_wav_file function");

    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Int,
    };
    println!("[ENCODER] --> 12. WAV spec created");

    let mut wav_buffer = Vec::new();
    {
        let mut writer = WavWriter::new(Cursor::new(&mut wav_buffer), spec).unwrap();
        println!("[ENCODER] --> 13. WavWriter initialized");

        for &sample in &audio_data {
            writer.write_sample(sample).unwrap();
        }
        println!("[ENCODER] --> 14. Audio data written to WAV buffer");

        writer.finalize().unwrap();
        println!("[ENCODER] --> 15. WAV file finalized");
    }

    println!("[ENCODER] --> 16. Returning WAV file data.\n");
    Ok(warp::reply::with_header(
        wav_buffer,
        "Content-Type",
        "audio/wav",
    ))
}
