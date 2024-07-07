use hound;
use std::f32::consts::PI;
use base64::{engine::general_purpose, Engine as _};

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

/// Decodes a WAV file that was encoded using Frequency Shift Keying (FSK)
/// and returns a Result containing either the decoded message as a String or an error
pub fn decode_file(input_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Open and read the WAV file
    let mut reader = hound::WavReader::open(input_file)?;
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    // Decode the audio samples into bits
    let mut bits = Vec::new();
    for chunk in samples.chunks(SAMPLES_PER_BIT as usize) {
        // Use Goertzel algorithm to detect the presence of each frequency
        let power_0 = goertzel(chunk, FREQUENCY_0);
        let power_1 = goertzel(chunk, FREQUENCY_1);
        // Determine which frequency is stronger and push the corresponding bit
        bits.push(if power_1 > power_0 { 1 } else { 0 });
    }

    // Convert bits to bytes
    let bytes: Vec<u8> = bits.chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &b| (acc << 1) | b))
        .collect();

    // Convert bytes to a string (which is Base64 encoded)
    let decoded = String::from_utf8(bytes)?;
    // Decode the Base64 string
    let json_message = general_purpose::STANDARD.decode(decoded)?;
    // Convert the decoded bytes to a JSON string
    let json_string: String = String::from_utf8(json_message)?;

    println!("[DECODER]: File decoded successfully");
    Ok(json_string)
}
