use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use super::constants::*;
use super::debuger::print_bits;

pub fn encode_message(message: &str, output_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[ENCODER]: Encoding message");

    // Convert message to bytes
    let message_bytes = message.as_bytes();

    // Encode to Base64
    let encoded = general_purpose::STANDARD.encode(message_bytes);

    // Convert Base64 string to bits
    let bits: Vec<bool> = encoded.bytes().flat_map(|byte| {
        (0..8).rev().map(move |i| (byte & (1 << i)) != 0)
    }).collect();

    // Construct the final bit sequence with preamble and postamble
    let mut encoded_bits: Vec<bool> = Vec::new();
    encoded_bits.extend(&PREAMBLE);
    encoded_bits.extend(&bits);
    encoded_bits.extend(&POSTAMBLE);

    // Debug print
    print_bits(&encoded_bits);

    // Convert bits to bytes for file writing
    let encoded_bytes: Vec<u8> = encoded_bits.chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &bit| (acc << 1) | bit as u8))
        .collect();

    println!("[ENCODER]: Saving encoded message to file");
    // Save encoded message to a file
    let mut file = File::create(output_file)?;
    file.write_all(&encoded_bytes)?;

    let file_path = Path::new(output_file).canonicalize()?
        .to_str().ok_or("[ENCODER]: Invalid file path")?.to_string();

    Ok(file_path)
}
