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

    let destination = decode_address(&bytes[0..7]);
    let source = decode_address(&bytes[7..14]);
    println!("[DECODER] --> 7. Decoded addresses: Destination: {}, Source: {}", destination, source);

    // Find end of address field (marked by LSB set to 1)
    let mut info_start = 14;
    while info_start < bytes.len() && (bytes[info_start - 1] & 0x01) == 0 {
        info_start += 7;
    }
    println!("[DECODER] --> 8. Found info_start at index: {}", info_start);

    if info_start + 2 >= bytes.len() {
        println!("[DECODER] --> 9. No information field");
        return Err("No information field".to_string());
    }

    // Skip control and protocol ID fields
    info_start += 2;

    // Extract information field (excluding CRC)
    let info_end = bytes.len() - 2;
    let information = String::from_utf8_lossy(&bytes[info_start..info_end]).to_string();
    println!("[DECODER] --> 9. Extracted information field: {}", information);

    // Verify CRC
    let crc = Crc::<u16>::new(&CRC_16_IBM_SDLC);
    let crc_range = &bytes[info_start..info_end];
    println!("[DECODER] --> CRC calculation range: {:02X?}", crc_range);
    let calculated_crc = crc.checksum(crc_range);
    let packet_crc = u16::from_le_bytes([bytes[info_end], bytes[info_end + 1]]);
    println!("[DECODER] --> 10. CRC check: calculated {:04X}, found {:04X}", calculated_crc, packet_crc);

    if calculated_crc != packet_crc {
        println!("[DECODER] --> 11. CRC check failed");
        return Err(format!("CRC mismatch: calculated {:04X}, found {:04X}", calculated_crc, packet_crc));
    }

    println!("[DECODER] --> 11. CRC check passed");

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
