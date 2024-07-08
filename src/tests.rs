use super::*;
use crate::helpers::{encoder, decoder};
use tempfile::NamedTempFile;

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
fn test_very_long_message() {
    let input = "a".repeat(10000);  // 10,000 character string
    let result = encode_decode(&input).unwrap();
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
