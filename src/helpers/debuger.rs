use base64::{engine::general_purpose, Engine as _};

// Helper function to print bits with detailed formatting (for debugging)
#[allow(dead_code)]
pub fn print_bits(bits: &[bool]) {
    println!("\n===========================");
    println!("Encoding Debug Information:\n");

    // Print Preamble
    println!("Preamble:");
    for &bit in &bits[0..16] {
        print!("{}", if bit { "1" } else { "0" });
    }
    println!("\n");

    // Print Data Size (32 bits after preamble)
    println!("Data Size:");
    for &bit in &bits[16..48] {
        print!("{}", if bit { "1" } else { "0" });
    }

    let data_size = bits[16..48].iter().enumerate().fold(0, |acc, (i, &bit)| {
        acc | ((bit as usize) << (31 - i))
    });
    println!("or : {} bits \n", data_size);


    // Print Data (in groups of 8 for readability)
    println!("Data:");
    let data_bits = &bits[48..bits.len() - 16];
    for (i, &bit) in data_bits.iter().enumerate() {
        print!("{}", if bit { "1" } else { "0" });
        if (i + 1) % 8 == 0 { print!(" "); }
        if (i + 1) % 64 == 0 { println!(); }
    }
    println!("\n");

    // Print Postamble
    println!("Postamble:");
    for &bit in &bits[bits.len() - 16..] {
        print!("{}", if bit { "1" } else { "0" });
    }
    println!("\n");

    // Convert data bits to bytes
    let data_bytes: Vec<u8> = data_bits.chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b as u8))
        .collect();

    // Convert bytes to Base64 string
    let base64_string = general_purpose::STANDARD.encode(&data_bytes);

    println!("\nBase64 Encoded:");
    println!("{}", base64_string);

    // Decode Base64 to get original message
    match general_purpose::STANDARD.decode(&base64_string) {
        Ok(decoded_bytes) => {
            match String::from_utf8(decoded_bytes) {
                Ok(original_message) => {
                    println!("\nDecoded Original Message:");
                    println!("{}", original_message);
                },
                Err(_) => println!("Error: Decoded bytes are not valid UTF-8"),
            }
        },
        Err(_) => println!("Error: Invalid Base64 encoding"),
    }

    println!("===========================");
    println!("\n");
}
