use std::fs::File;
use std::io::Write;
use std::path::Path;
use super::constants::*;
use super::debuger::print_symbols;

pub fn encode_message(message: &str, output_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[ENCODER]: Starting to encode message");

    // Convert string to bytes
    let message_bytes = message.as_bytes();

    // Calculate the size of the data (in 2-bit symbols)
    let data_size = message_bytes.len() * 4; // Each byte becomes 4 2-bit symbols

    // Convert the size to 2-bit symbols
    let size_symbols: Vec<u8> = (0..SIZE_BITS/2).rev()
        .map(|i| ((data_size >> (i * 2)) & 0b11) as u8)
        .collect();

    println!("[ENCODER]: Data size: {} symbols", data_size);

    // Construct the final symbol sequence
    let mut encoded_symbols: Vec<u8> = Vec::new();
    encoded_symbols.extend(&PREAMBLE);
    encoded_symbols.extend(&size_symbols);

    // Convert message bytes to 2-bit symbols
    for &byte in message_bytes {
        encoded_symbols.push((byte >> 6) & 0b11);
        encoded_symbols.push((byte >> 4) & 0b11);
        encoded_symbols.push((byte >> 2) & 0b11);
        encoded_symbols.push(byte & 0b11);
    }

    encoded_symbols.extend(&POSTAMBLE);

    println!("[ENCODER]: Total encoded symbols: {}", encoded_symbols.len());

    // Debug print
    print_symbols(&encoded_symbols);

    println!("[ENCODER]: Saving encoded message to file");
    // Save encoded symbols directly to the file
    let mut file = File::create(output_file)?;
    file.write_all(&encoded_symbols)?;

    let file_path = Path::new(output_file).canonicalize()?
        .to_str().ok_or("[ENCODER]: Invalid file path")?.to_string();

    println!("[ENCODER]: Encoding completed successfully");
    Ok(file_path)
}
