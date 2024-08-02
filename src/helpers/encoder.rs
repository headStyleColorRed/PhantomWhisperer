use crc16::*;
use crate::helpers::constants::*;
use std::f32::consts::PI;
use crate::helpers::aprs_packet::AprsPacket;

impl AprsPacket {
    pub fn encode(&self) -> Vec<u8> {
        let mut packet = Vec::new();

        // Add addresses
        packet.extend(encode_address(&self.destination, false));
        packet.extend(encode_address(&self.source, true));

        // Control field and Protocol ID
        packet.push(0x03); // Control field: UI-frame
        packet.push(0xf0); // Protocol ID: no layer 3

        // Information field
        packet.extend(self.information.as_bytes());

        // Calculate and append CRC
        let crc = State::<CCITT_FALSE>::calculate(&packet);
        packet.extend(&crc.to_le_bytes());

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

/// Prepares APRS packets from a message, splitting it into chunks if necessary
/// Returns a vector of encoded APRS packets (each as a vector of bytes)
pub fn prepare_packets(source: &str, destination: &str, message: &str) -> Vec<Vec<u8>> {
    let mut packets = Vec::new();
    // Split the message into chunks of MAX_PAYLOAD_SIZE
    let chunks = message.as_bytes().chunks(MAX_PAYLOAD_SIZE);
    let total_chunks = chunks.len();

    for (i, chunk) in chunks.enumerate() {
        let info = if total_chunks > 1 {
            // If multiple chunks, add sequence number
            format!("{{{}:{}}}{}", i + 1, total_chunks, String::from_utf8_lossy(chunk))
        } else {
            String::from_utf8_lossy(chunk).into_owned()
        };

        // Create and encode an APRS packet for each chunk
        let packet = AprsPacket::new(destination, source, &info);
        packets.push(packet.encode());
    }

    packets
}

/// Encodes a message into AFSK modulated audio samples
/// Returns a vector of i16 audio samples representing the encoded message
pub fn encode_message(source: &str, destination: &str, message: &str) -> Vec<i16> {
    // Prepare the APRS packets
    let packets = prepare_packets(source, destination, message);
    let mut audio_samples = Vec::new();

    // Modulate each packet into audio samples
    for packet in packets {
        audio_samples.extend(afsk_modulate(&packet));
    }

    audio_samples
}

/// Modulates a byte array into AFSK audio samples
/// Returns a vector of i16 audio samples representing the modulated data
fn afsk_modulate(data: &[u8]) -> Vec<i16> {
    let samples_per_bit = (SAMPLE_RATE as f32 / BAUD_RATE) as usize;
    let mut audio_samples = Vec::new();

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
