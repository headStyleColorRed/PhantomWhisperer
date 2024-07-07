use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn encode_message(message: &str, output_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[ENCODER]: Encoding message");

    // Convert message to bytes
    let message_bytes = message.as_bytes();

    // Encode to Base64
    let encoded = general_purpose::STANDARD.encode(message_bytes);

    println!("[ENCODER]: Saving message to .wav file");
    // Save encoded message to a file
    let mut file = File::create(output_file)?;
    file.write_all(encoded.as_bytes())?;

    let file_path = Path::new(output_file).canonicalize()?.to_str().ok_or("[ENCODER]: Invalid file path")?.to_string();

    Ok(file_path)
}
