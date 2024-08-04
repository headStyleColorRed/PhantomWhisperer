use crc::{Crc, CRC_16_IBM_SDLC};
use crate::helpers::constants::*;
use std::f32::consts::PI;
use crate::models::aprs_packet::AprsPacket;
use std::io::Cursor;
use hound::{WavWriter, WavSpec};
use warp::reject::Rejection;

// https://hugosprojects.wordpress.com/2014/03/15/implementing-aprs/
// | Field Name            | Number of Bytes | Example    |
// |-----------------------|-----------------|------------|
// | Flag                  | 1               | 0x7E       |
// | Dest Address          | 7               | "M6CYT 1"  |
// | Source Address        | 7               | "M6CYT 7"  |
// | Digipeater Addresses  | 0-56            | ""         |
// | Control Field         | 1               | 0x03       |
// | Protocol ID           | 1               | 0xF0       |
// | Information Field     | 1-256           | "Hello!"   |
// | Frame Check Sequence  | 2               | —          |
// | Flag                  | 1               | 0x7E       |
// |-----------------------|-----------------|------------|


/// Encodes a message into AFSK modulated audio samples
/// Returns a vector of i16 audio samples representing the encoded message
pub fn encode_message(source: &str, destination: &str, digipeaters: &Vec<String>, information: &str) -> Vec<i16> {
    println!("[ENCODER] --> 2. Encoding message from {} to {}", source, destination);
    // Prepare the APRS packets
    let packets = prepare_packets(source, destination, digipeaters, information);
    let mut audio_samples = Vec::new();

    // Modulate each packet into audio samples
    for (i, packet) in packets.iter().enumerate() {
        println!("[ENCODER] --> 8. Modulating packet {} of {}", i + 1, packets.len());
        audio_samples.extend(afsk_modulate(packet));
    }

    audio_samples
}


/// Prepares APRS packets from a message, splitting it into chunks if necessary
/// Returns a vector of encoded APRS packets (each as a vector of bytes)
pub fn prepare_packets(source: &str, destination: &str, digipeaters: &Vec<String>, information: &str) -> Vec<Vec<u8>> {
    println!("[ENCODER] --> 3. Preparing packets for message: {}", information);
    let mut packets = Vec::new();
    // Split the message into chunks of MAX_PAYLOAD_SIZE
    let chunks = information.as_bytes().chunks(MAX_PAYLOAD_SIZE);
    let total_chunks = chunks.len();

    for (i, chunk) in chunks.enumerate() {
        println!("[ENCODER] --> 4. Processing chunk {} of {}", i + 1, total_chunks);
        if total_chunks > 1 {
            // If multiple chunks, add sequence number
            format!("{{{}:{}}}{}", i + 1, total_chunks, String::from_utf8_lossy(chunk))
        } else {
            String::from_utf8_lossy(chunk).into_owned()
        };

        // Create and encode an APRS packet for each chunk
        let packet = AprsPacket::new(source, destination, digipeaters, information);
        packets.push(packet.encode());
    }

    packets
}

impl AprsPacket {
    pub fn encode(&self) -> Vec<u8> {
        println!("[ENCODER] --> 5. Encoding APRS packet");
        let mut packet = Vec::new();

        // Add starting flag which will allow the decoder to find the start of the packet
        packet.push(FLAG);

        // Add addresses
        packet.extend(encode_address(&self.destination, false));
        packet.extend(encode_address(&self.source, true));

        println!("[ENCODER] --> 6. Encoding Destination: {}", self.destination);
        println!("[ENCODER] --> 7. Encoding Source: {}", self.source);


        // Add Digipeater addresses
        for (i, digipeater) in self.digipeaters.iter().enumerate() {
            println!("[ENCODER] --> 8. Encoding Digipeater: {}", digipeater);
            let is_last = i == self.digipeaters.len() - 1;
            packet.extend(encode_address(digipeater, is_last));
        }

        // Control field and Protocol ID
        packet.push(0x03); // Control field: UI-frame
        packet.push(0xf0); // Protocol ID: no layer 3

        // Information field
        let info_field = self.information.as_bytes();
        packet.extend(info_field);

        // Calculate CRC only on the information field
        let crc = Crc::<u16>::new(&CRC_16_IBM_SDLC);
        let calculated_crc = crc.checksum(info_field);
        packet.extend(&calculated_crc.to_le_bytes());

        // Add ending flag to mark the end of the packet
        packet.push(FLAG);

        println!("[ENCODER] --> 6. CRC calculated: {:04X}, appended to packet", calculated_crc);

        packet
    }
}

/// Encodes an APRS address (callsign-SSID) into the AX.25 format
/// Returns a vector of bytes representing the encoded address
pub fn encode_address(address: &str, last: bool) -> Vec<u8> {
    let mut encoded = Vec::new();
    // Split the address into callsign and SSID parts
    let parts: Vec<&str> = address.split('-').collect();
    let callsign = parts[0].to_uppercase();
    // Parse SSID, default to 0 if not present or invalid
    let ssid = parts.get(1).unwrap_or(&"0").parse::<u8>().unwrap_or(0);

    // Encode each character of the callsign
    for (i, byte) in callsign.as_bytes().iter().enumerate() {
        if i < 6 {
            // Shift each byte left by 1 bit (AX.25 requirement)
            encoded.push(byte << 1);
        }
    }

    // Pad the callsign to 6 bytes if necessary
    while encoded.len() < 6 {
        encoded.push(b' ' << 1);
    }

    // Construct the SSID byte
    let mut ssid_byte = 0b01100000 | (ssid << 1);
    if last {
        // Set the least significant bit for the last address
        ssid_byte |= 1;
    }
    encoded.push(ssid_byte);

    encoded
}

/// Modulates a byte array into AFSK audio samples
/// Returns a vector of i16 audio samples representing the modulated data
fn afsk_modulate(data: &[u8]) -> Vec<i16> {
    println!("[ENCODER] --> 9. AFSK modulating {} bytes", data.len());
    let samples_per_bit = (SAMPLE_RATE as f32 / BAUD_RATE) as usize;
    let mut audio_samples = Vec::new();

    println!("[ENCODER] --> 10. Generating tones for each bit");
    for &byte in data {
        for bit in (0..8).rev() {
            // Choose frequency based on the bit value using the modulation of Bell 202 tones
            let freq = if (byte & (1 << bit)) != 0 { MARK_FREQ } else { SPACE_FREQ };
            audio_samples.extend(generate_tone(freq, samples_per_bit));
        }
    }

    audio_samples
}

/// Generates a tone at a specified frequency
/// Returns a vector of i16 audio samples representing the generated tone
fn generate_tone(freq: f32, num_samples: usize) -> Vec<i16> {
    (0..num_samples)
        .map(|i| {
            let t = i as f32 / SAMPLE_RATE as f32;
            // Generate sine wave and scale to i16 range
            (2.0 * PI * freq * t).sin() * i16::MAX as f32
        })
        .map(|sample| sample as i16)
        .collect()
}


pub fn create_wav_file(audio_data: Vec<i16>) -> Result<impl warp::Reply, Rejection> {
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
