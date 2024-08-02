use crate::helpers::encoder;
use std::io::Cursor;
use warp::reject::Rejection;
use crate::helpers::aprs_packet::AprsPacket;
use hound::{WavWriter, WavSpec};
use crate::helpers::constants::*;

// This method will be called when the client sends a POST request to /encoder
// The request body should contain a JSON object with the following fields:
// - destination: the destination callsign
// - source: the source callsign
// - information: the message to be transmitted
pub async fn create_packet(data: AprsPacket) -> Result<impl warp::Reply, Rejection> {
    // Encode the message
    let encoded_packet: Vec<i16> = encoder::encode_message(&data.information, &data.source, &data.destination);

    create_wav_file(encoded_packet)
}

fn create_wav_file(audio_data: Vec<i16>) -> Result<impl warp::Reply, Rejection> {
    // Define WAV specifications
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Int,
    };

    // Create a buffer to store the WAV data
    let mut wav_buffer = Vec::new();
    {
        let mut writer = WavWriter::new(Cursor::new(&mut wav_buffer), spec).unwrap();

        // Write audio data to the WAV file
        for &sample in &audio_data {
            writer.write_sample(sample).unwrap();
        }

        writer.finalize().unwrap();
    }

    // Return the WAV file data
    Ok(warp::reply::with_header(
        wav_buffer,
        "Content-Type",
        "audio/wav",
    ))
}
