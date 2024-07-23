use std::fs::File;
use std::io::Write;
use std::path::Path;
use super::constants::*;
use super::debuger::print_bits;

pub fn encode_message(message: &str, output_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[ENCODER]: Encoding message");

    // Convert Base64 string to bits
    let bits: Vec<bool> = message.bytes().flat_map(|byte| {
        (0..8).rev().map(move |i| (byte & (1 << i)) != 0)
    }).collect();

    // Calculate the size of the data (in bits)
    let data_size = bits.len();

    // Convert the size to a bit vector
    let size_bits: Vec<bool> = (0..SIZE_BITS).rev()
        .map(|i| (data_size & (1 << i)) != 0)
        .collect();

    // Construct the final bit sequence with preamble and postamble
    let mut encoded_bits: Vec<bool> = Vec::new();
    encoded_bits.extend(&PREAMBLE);
    encoded_bits.extend(&size_bits);
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
