use super::constants::*;
use crate::helpers::aprs_packet::AprsPacket;
use crc::{Crc, CRC_16_IBM_SDLC};
use rustfft::{FftPlanner, num_complex::Complex};
use bytes::Buf;
use futures::TryStreamExt;
use hound::WavReader;
use crate::helpers::errors::CustomError;
use warp::reject::Rejection;

pub fn decode_audio(samples: &[i16]) -> Result<AprsPacket, String> {
    let bits = demodulate_afsk(samples);
    let bytes = bits_to_bytes(&bits);
    parse_aprs_packet(&bytes)
}

fn demodulate_afsk(samples: &[i16]) -> Vec<bool> {
    let mut bits = Vec::new();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(SAMPLES_PER_BIT);

    for chunk in samples.chunks(SAMPLES_PER_BIT) {
        let mut buffer: Vec<Complex<f32>> = chunk.iter()
            .map(|&s| Complex::new(s as f32, 0.0))
            .collect();

        fft.process(&mut buffer);

        let mark_bin = (MARK_FREQ * SAMPLES_PER_BIT as f32 / SAMPLE_RATE as f32).round() as usize;
        let space_bin = (SPACE_FREQ * SAMPLES_PER_BIT as f32 / SAMPLE_RATE as f32).round() as usize;

        let mark_energy = buffer[mark_bin].norm_sqr();
        let space_energy = buffer[space_bin].norm_sqr();

        bits.push(mark_energy > space_energy);
    }

    bits
}

fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| chunk.iter().fold(0, |acc, &b| (acc << 1) | (b as u8)))
        .collect()
}

fn parse_aprs_packet(bytes: &[u8]) -> Result<AprsPacket, String> {
    println!("[DECODER] --> 6. Starting parse_aprs_packet");
    if bytes.len() < 16 {
        return Err("Packet too short".to_string());
    }

    // Find start flag in byte stream
    let mut packet_byte_position = 0;

    // Iterate through the bytes until the flag is found
    while packet_byte_position < bytes.len() && bytes[packet_byte_position] != FLAG {
        packet_byte_position += 1;
    }

    // If no flag is found, return an error
    if packet_byte_position == bytes.len() {
        return Err("No start flag found".to_string());
    } else {
        println!("[DECODER] --> 7. Found start flag at index: {}", packet_byte_position);
        packet_byte_position += 1;
    }

    // Decode addres Destination
    let destination = decode_address(&bytes[packet_byte_position..packet_byte_position + 7]);

    // Advance packet byte position
    packet_byte_position += 7;

    let source = decode_address(&bytes[packet_byte_position..packet_byte_position + 7]);
    println!("[DECODER] --> 8. Decoded addresses: Destination: {}, Source: {}", destination, source);

    // Advance packet byte position
    packet_byte_position += 7;

    // Iterate through the bytes until the LSB is found
    while packet_byte_position < bytes.len() && (bytes[packet_byte_position - 1] & 0x01) == 0 {
        packet_byte_position += 7;
    }
    println!("[DECODER] --> 9. Found packet_byte_position at index: {}", packet_byte_position);

    if packet_byte_position + 2 >= bytes.len() {
        println!("[DECODER] --> 10. No information field");
        return Err("No information field".to_string());
    }

    // Skip control and protocol ID fields
    packet_byte_position += 2;

    // Extract information field (excluding CRC and ending flag)
    let info_end = bytes.len() - 3;
    let information = String::from_utf8_lossy(&bytes[packet_byte_position..info_end]).to_string();
    println!("[DECODER] --> 10. Extracted information field: {}", information);

    // Verify CRC
    let crc = Crc::<u16>::new(&CRC_16_IBM_SDLC);
    let crc_range = &bytes[packet_byte_position..info_end];
    let calculated_crc = crc.checksum(crc_range);
    let packet_crc = u16::from_le_bytes([bytes[info_end], bytes[info_end + 1]]);
    println!("[DECODER] --> 11. CRC check: calculated {:04X}, found {:04X}", calculated_crc, packet_crc);

    if calculated_crc != packet_crc {
        println!("[DECODER] --> 12. CRC check failed");
        return Err(format!("CRC mismatch: calculated {:04X}, found {:04X}", calculated_crc, packet_crc));
    }

    println!("[DECODER] --> 12. CRC check passed");

    // Check for ending flag
    if bytes[info_end + 2] != FLAG {
        return Err("No ending flag found".to_string());
    } else {
        println!("[DECODER] --> 13. Found ending flag");
    }

    Ok(AprsPacket {
        destination,
        source,
        information,
    })
}

fn decode_address(bytes: &[u8]) -> String {
    let callsign: String = bytes[0..6]
        .iter()
        .map(|&b| (b >> 1) as char)
        .collect::<String>()
        .trim()
        .to_string();

    let ssid = (bytes[6] >> 1) & 0x0F;

    if ssid == 0 {
        callsign
    } else {
        format!("{}-{}", callsign, ssid)
    }
}

pub async fn extract_wav_from_multipart(mut form: warp::multipart::FormData) -> Result<Vec<i16>, Rejection> {
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

    Ok(samples)

}
