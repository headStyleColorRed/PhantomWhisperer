use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::Write;

pub fn encode_message(message: &str, output_file: &str) -> Result<(), std::io::Error> {
    println!("Encoding message: {}", message);

    // Convert message to bytes
    let message_bytes = message.as_bytes();

    // Encode to Base64
    let encoded = general_purpose::STANDARD.encode(message_bytes);

    println!("Encoded message: {}", encoded);

    // Save encoded message to a file
    let mut file = File::create(output_file)?;
    file.write_all(encoded.as_bytes())?;

    println!("Encoded message saved to: {}", output_file);
    Ok(())
}
