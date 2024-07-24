
use super::*;
use crate::helpers::{encoder, decoder, modulator};
use tempfile::NamedTempFile;
use hound::{self};
use super::helpers::constants::*;
pub const SIZE_SYMBOLS: usize = 16;

// H E L P E R    F U N C T I O N S
//
// The encode_decode function takes a string input, encodes it, modulates it, and then decodes it.
fn encode_decode(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create temporary files
    let encoded_file = NamedTempFile::new()?;
    let modulated_file = NamedTempFile::new()?;

    // Encode and modulate
    encoder::encode_message(input, encoded_file.path().to_str().unwrap())?;
    helpers::modulator::modulate_file(encoded_file.path().to_str().unwrap(), modulated_file.path().to_str().unwrap())?;

    // Decode
    let decoded = decoder::decode_file(modulated_file.path().to_str().unwrap())?;

    Ok(decoded)
}

#[test]
fn test_empty_message() {
    let input = "";
    let result = encode_decode(input).unwrap();
    assert_eq!(input, result);
}

#[test]
fn test_short_message() {
    let input = "Hello, World!";
    let result = encode_decode(input).unwrap();
    assert_eq!(input, result);
}

#[test]
fn test_long_message() {
    let input = "This is a longer message that will test the encoding and decoding process with a substantial amount of text. It includes various characters and punctuation!";
    let result = encode_decode(input).unwrap();
    assert_eq!(input, result);
}

#[test]
fn test_special_characters() {
    let input = "Special chars: !@#$%^&*()_+-=[]{}|;:',.<>?`~";
    let result = encode_decode(input).unwrap();
    assert_eq!(input, result);
}

#[test]
fn test_unicode_characters() {
    let input = "Unicode: 你好, Здравствуйте, こんにちは, 안녕하세요";
    let result = encode_decode(input).unwrap();
    assert_eq!(input, result);
}

#[test]
fn test_encode_error() {
    let result = encoder::encode_message("test", "/nonexistent/path/file.txt");
    assert!(result.is_err());
}

#[test]
fn test_decode_error() {
    let result = decoder::decode_file("/nonexistent/path/file.wav");
    assert!(result.is_err());
}

#[test]
fn test_preamble_postamble() {
    let input = "Test message";
    let encoded_file = NamedTempFile::new().unwrap();
    encoder::encode_message(input, encoded_file.path().to_str().unwrap()).unwrap();

    let encoded_data = std::fs::read(encoded_file.path()).unwrap();

    // Now each byte in encoded_data directly represents a symbol
    let symbols: Vec<u8> = encoded_data;

    assert_eq!(&symbols[..PREAMBLE.len()], PREAMBLE, "Preamble not found at the start of encoded data");
    assert_eq!(&symbols[symbols.len() - POSTAMBLE.len()..], POSTAMBLE, "Postamble not found at the end of encoded data");

    // Additional check to ensure the preamble and postamble are not just a series of zeros
    assert!(PREAMBLE.iter().any(|&x| x != 0), "Preamble should not be all zeros");
    assert!(POSTAMBLE.iter().any(|&x| x != 0), "Postamble should not be all zeros");

    // Check that all symbols are valid (0-3)
    assert!(symbols.iter().all(|&s| s <= 3), "All symbols should be in the range 0-3");

    // Verify the data portion
    let data_start = PREAMBLE.len() + SIZE_SYMBOLS;
    let data_end = symbols.len() - POSTAMBLE.len();
    let data_symbols = &symbols[data_start..data_end];

    // Convert data symbols back to bytes
    let decoded_bytes: Vec<u8> = data_symbols.chunks(4)
        .map(|chunk| {
            chunk.iter().enumerate().fold(0u8, |acc, (i, &sym)| {
                acc | (sym << (6 - i * 2))
            })
        })
        .collect();

    let decoded_message = String::from_utf8(decoded_bytes).unwrap();
    assert_eq!(decoded_message, input, "Decoded message does not match input");

    println!("Encoded symbols: {:?}", symbols);
    println!("Preamble: {:?}", PREAMBLE);
    println!("Postamble: {:?}", POSTAMBLE);
    println!("Decoded message: {}", decoded_message);
}

#[test]
fn test_size_encoding() {
    let input = "Test message for size encoding";
    let result = encode_decode(input).unwrap();
    assert_eq!(input, result, "Size encoding/decoding failed");
}

#[test]
fn test_noise_resistance() {
    let input = "Test message for noise resistance";
    let encoded_file = NamedTempFile::new().unwrap();
    let modulated_file = NamedTempFile::new().unwrap();
    let noisy_file = NamedTempFile::new().unwrap();

    encoder::encode_message(input, encoded_file.path().to_str().unwrap()).unwrap();
    modulator::modulate_file(encoded_file.path().to_str().unwrap(), modulated_file.path().to_str().unwrap()).unwrap();

    // Add some noise to the modulated file
    let mut reader = hound::WavReader::open(modulated_file.path()).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let spec = reader.spec();

    let noisy_samples: Vec<i16> = samples.iter().map(|&s| {
        let noise = (rand::random::<f32>() - 0.5) * 1000.0;  // Add random noise between -500 and 500
        (s as f32 + noise).clamp(i16::MIN as f32, i16::MAX as f32) as i16
    }).collect();

    let mut noisy_writer = hound::WavWriter::create(noisy_file.path(), spec).unwrap();
    for &sample in &noisy_samples {
        noisy_writer.write_sample(sample).unwrap();
    }
    noisy_writer.finalize().unwrap();

    let result = decoder::decode_file(noisy_file.path().to_str().unwrap()).unwrap();
    assert_eq!(input, result, "Decoding with noise failed");
}

#[test]
fn test_different_sample_rates() {
    let input = "Test message for different sample rates";
    let encoded_file = NamedTempFile::new().unwrap();
    let modulated_file = NamedTempFile::new().unwrap();

    encoder::encode_message(input, encoded_file.path().to_str().unwrap()).unwrap();

    // Modify the modulate_file function to accept a sample rate parameter
    modulator::modulate_file(encoded_file.path().to_str().unwrap(), modulated_file.path().to_str().unwrap()).unwrap();

    let result = decoder::decode_file(modulated_file.path().to_str().unwrap()).unwrap();
    assert_eq!(input, result, "Encoding/decoding with different sample rate failed");
}

#[test]
fn test_performance() {
    use std::time::Instant;

    let input = "a".repeat(10000);  // 1000 character string
    let start = Instant::now();
    let _ = encode_decode(&input).unwrap();
    let duration = start.elapsed();

    println!("Time taken to encode, modulate, and decode 1000 characters: {:?}", duration);
    assert!(duration.as_secs() < 10, "Performance test failed: took longer than 10 seconds");
}

#[test]
fn test_fuzz() {
    use rand::Rng;

    for _ in 0..50 {  // Run 50 fuzz tests
        let length = rand::thread_rng().gen_range(0..1000);
        let input: String = (0..length)
            .map(|_| rand::thread_rng().gen_range(0..255) as u8 as char)
            .collect();

        let result = encode_decode(&input);
        assert!(result.is_ok(), "Fuzz test failed for input: {:?}", input);
        assert_eq!(input, result.unwrap(), "Fuzz test failed: input and output don't match");
    }
}
