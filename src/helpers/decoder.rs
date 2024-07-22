use hound;
use std::f32::consts::PI;
use base64::{engine::general_purpose, Engine as _};
use super::constants::*;
use super::debuger::*;
use std::collections::VecDeque;

// Constants for audio processing
const SAMPLE_RATE: f32 = 44100.0;  // Standard CD-quality audio sample rate
const SAMPLES_PER_BIT: u32 = 100;  // Number of samples used to represent each bit
const FREQUENCY_0: f32 = 1000.0;  // Frequency used to represent bit 0
const FREQUENCY_1: f32 = 2000.0;  // Frequency used to represent bit 1

/// Implements the Goertzel algorithm to detect the presence of a specific frequency in a signal
/// and returns a f32 value representing the magnitude of the target frequency in the sample
fn goertzel(samples: &[i16], target_frequency: f32) -> f32 {
    // Calculate the normalized frequency
    let omega = 2.0 * PI * target_frequency / SAMPLE_RATE;
    // Precompute the coefficient for the algorithm
    let coeff = 2.0 * omega.cos();
    // Initialize the algorithm's state variables
    let (mut s1, mut s2) = (0.0, 0.0);

    // Process each sample through the algorithm
    for &sample in samples {
        let s0 = sample as f32 + coeff * s1 - s2;
        s2 = s1;
        s1 = s0;
    }

    // Compute and return the magnitude of the frequency component
    (s1 * s1 + s2 * s2 - coeff * s1 * s2).sqrt()
}

/// Detects whether a chunk of samples represents a '0' bit or a '1' bit
/// by analizyin which frequency (FREQUENCY_0 or FREQUENCY_1) is more prevalent in the given chunk of samples.
fn detect_bit(chunk: &[i16]) -> bool {
    let power_0 = goertzel(chunk, FREQUENCY_0);
    let power_1 = goertzel(chunk, FREQUENCY_1);
    power_1 > power_0  // If power of FREQUENCY_1 is greater, it's a '1' bit
}

/// Compares a detected bit pattern with an expected pattern
///
/// This function calculates the percentage of matching bits between
/// the detected pattern and the expected pattern.
///
/// @param detected: Slice of detected bits
/// @param pattern: Slice of expected bit pattern
/// @return: A float between 0 and 1 representing the match percentage
fn compare_pattern(detected: &[bool], pattern: &[bool]) -> f32 {
    let matches = detected.iter()
        .zip(pattern.iter())
        .filter(|&(a, b)| a == b)
        .count();
    matches as f32 / pattern.len() as f32
}

#[derive(Debug, PartialEq)]
enum DecoderState {
    SearchingPreamble,
    DecodingData
}

pub fn decode_file(input_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[DECODER] Starting to decode file: {}", input_file);

    // Open the WAV file and read all samples into a vector
    let mut reader = hound::WavReader::open(input_file)?;
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    println!("[DECODER] Read {} samples from file.", samples.len());

    let mut decoded_bits = Vec::new();  // Store the final decoded bits
    let mut current_position = 0;  // Keep track of our position in the samples
    let mut state = DecoderState::SearchingPreamble;  // Start in the preamble search state
    let mut bit_buffer = VecDeque::new();  // Sliding window of recent bits

    // Main decoding loop
    while current_position < samples.len() {

        // Detect the current bit if we have enough samples left
        if current_position + SAMPLES_PER_BIT as usize <= samples.len() {
            let chunk = &samples[current_position..current_position + SAMPLES_PER_BIT as usize];
            let bit = detect_bit(chunk);  // Determine if this chunk represents a 0 or 1
            bit_buffer.push_back(bit);  // Add the detected bit to our sliding window
            if bit_buffer.len() > 16 {
                bit_buffer.pop_front();  // Keep the buffer at a maximum of 16 bits
            }
        }

        // State machine for decoding
        match state {
            DecoderState::SearchingPreamble => {
                // Check if the current 16 bits match the preamble pattern
                if bit_buffer.len() == 16 && compare_pattern(&bit_buffer.make_contiguous(), &PREAMBLE) >= 0.9 {
                    println!("[DECODER]: Preamble detected");
                    decoded_bits.extend(&PREAMBLE);  // Add preamble to decoded bits
                    bit_buffer.clear();  // Clear the buffer to start fresh for data decoding
                    state = DecoderState::DecodingData;  // Move to data decoding state
                }
            },
            DecoderState::DecodingData => {
                // Check if we've reached the postamble
                if bit_buffer.len() == 16 && compare_pattern(&bit_buffer.make_contiguous(), &POSTAMBLE) >= 0.9 {
                    println!("[DECODER]: Postamble detected, decoding complete");
                    decoded_bits.extend(&POSTAMBLE);  // Add postamble to decoded bits
                    break;  // Exit the decoding loop
                } else if bit_buffer.len() == 16 {
                    // If we have 16 bits but it's not a postamble, the first bit must be data
                    decoded_bits.push(bit_buffer[0]);  // Add the first bit in the buffer to decoded data
                    bit_buffer.pop_front();  // Remove the first bit as we've processed it
                }
            }
        }

        // Move to the next bit
        current_position += SAMPLES_PER_BIT as usize;
    }

    // If decoded_bits is empty, the file was not encoded with our protocol
    println!("[DECODER] Decoded bits length: {}", decoded_bits.len());
    if decoded_bits.is_empty() {
        return Err("File does not contain encoded data".into());
    }

    // Print the decoded bits for debugging
    print_bits(&decoded_bits);

    // Convert bits to bytes, excluding preamble and postamble
    let bytes: Vec<u8> = decoded_bits[PREAMBLE.len()..decoded_bits.len() - POSTAMBLE.len()]
        .chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b as u8))
        .collect();
    println!("[DECODER] Converted bits to bytes: {:?}", bytes);

    // Convert bytes to a string (which is Base64 encoded)
    let decoded = String::from_utf8(bytes)?;
    println!("[DECODER] Converted bytes to Base64 string: {}", decoded);

    // Decode the Base64 string
    let json_message = general_purpose::STANDARD.decode(decoded)?;
    println!("[DECODER] Decoded Base64 string to JSON message.");

    // Convert the decoded bytes to a JSON string
    let json_string = String::from_utf8(json_message)?;
    println!("[DECODER] JSON message successfully converted to string.");

    println!("[DECODER] File decoded successfully");
    Ok(json_string)
}
