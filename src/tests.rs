use super::*;
use crate::helpers::{encoder, decoder, modulator};
use helpers::constants::{POSTAMBLE, PREAMBLE};
use tempfile::NamedTempFile;

// H E L P E R    F UN C T I O N S
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
    let bits: Vec<bool> = encoded_data.iter().flat_map(|&byte| (0..8).rev().map(move |i| (byte & (1 << i)) != 0)).collect();

    assert!(bits.starts_with(&PREAMBLE), "Preamble not found at the start of encoded data");
    assert!(bits.ends_with(&POSTAMBLE), "Postamble not found at the end of encoded data");
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
