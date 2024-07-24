use base64::{engine::general_purpose, Engine as _};

// Helper function to print bits with detailed formatting (for debugging)
#[allow(dead_code)]
pub fn print_symbols(symbols: &[u8]) {
    println!("\n===========================");
    println!("Encoding Debug Information:\n");

    // Print Preamble
    println!("Preamble:");
    for &symbol in &symbols[0..8] {
        print!("{:02b}", symbol);
    }
    println!("\n");

    // Print Data Size (16 symbols after preamble, representing 32 bits)
    println!("Data Size:");
    for &symbol in &symbols[8..24] {
        print!("{:02b}", symbol);
    }

    let data_size = symbols[8..24].iter().enumerate().fold(0, |acc, (i, &symbol)| {
        acc | ((symbol as usize) << (2 * (15 - i)))
    });
    println!(" or : {} symbols ({} bits)\n", data_size, data_size * 2);

    // Print Data (in groups of 4 symbols for readability)
    println!("Data:");
    let data_symbols = &symbols[24..symbols.len() - 8];
    for (i, &symbol) in data_symbols.iter().enumerate() {
        print!("{:02b}", symbol);
        if (i + 1) % 4 == 0 { print!(" "); }
        if (i + 1) % 32 == 0 { println!(); }
    }
    println!("\n");

    // Print Postamble
    println!("Postamble:");
    for &symbol in &symbols[symbols.len() - 8..] {
        print!("{:02b}", symbol);
    }
    println!("\n");

    // Convert data symbols to bytes
    let data_bytes: Vec<u8> = data_symbols.chunks(4)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &sym| (acc << 2) | sym))
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
